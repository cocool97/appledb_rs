use anyhow::{Result, bail};
use apple_codesign::{MachOBinary, path_is_macho};
use appledb_common::{IPSWEntitlements, IPSWExecutableEntitlements, config::ServerConfig};
use base64::{Engine, prelude::BASE64_STANDARD};
use plist::Value;
use std::{
    collections::HashSet,
    io::Cursor,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::{fs::File, io::AsyncReadExt};
use walkdir::WalkDir;

use crate::{models::EntSubCommands, server_controller::ServerController};

pub async fn parse_entitlements_command(
    configuration: ServerConfig,
    subcommand: EntSubCommands,
) -> Result<()> {
    match subcommand {
        EntSubCommands::Parse {
            mount_point,
            platform,
            version,
            model_code,
        } => {
            log::info!(
                "IPSW has platform={platform}, model_code={model_code} and version={version}"
            );
            let mut ipsw_entitlements = IPSWEntitlements::new(platform.into(), model_code, version);
            parse_entitlements(mount_point, &mut ipsw_entitlements).await?;
            log::info!("Sending entitlements to server...");
            let server_controller = ServerController::new(configuration.listen_mode)?;
            let response = server_controller
                .post_executable_entitlements(ipsw_entitlements)
                .await?;
            log::info!("Received response: {}", response);
            Ok(())
        }
        EntSubCommands::DumpEnt { executable_path } => {
            dump_executable_entitlements(executable_path).await
        }
    }
}

async fn parse_entitlements<P: AsRef<Path>>(
    mount_point: P,
    ipsw_entitlements: &mut IPSWEntitlements,
) -> Result<()> {
    for entry in WalkDir::new(&mount_point) {
        let entry = entry?;
        if let Ok(is_executable) = path_is_macho(entry.path()) {
            if is_executable {
                match parse_entitlements_file(entry.path()).await {
                    Ok(res) => {
                        if let Some(entitlements) = res {
                            let entry = entry.into_path();
                            let stripped_path = entry.strip_prefix(&mount_point)?;
                            let full_absolute_path = match &stripped_path.is_absolute() {
                                true => stripped_path.to_path_buf(),
                                false => PathBuf::from_str("/")?.join(stripped_path),
                            };
                            ipsw_entitlements.add_executable_entitlements(
                                full_absolute_path.to_string_lossy(),
                                entitlements,
                            );
                        }
                    }
                    Err(e) => log::error!("error with path {}: {e}", entry.into_path().display()),
                }
            }
        }
    }

    log::info!(
        "Number of executables: {}",
        ipsw_entitlements.executable_entitlements.len()
    );

    Ok(())
}

async fn dump_executable_entitlements<P: AsRef<Path>>(executable_path: P) -> Result<()> {
    match parse_entitlements_file(executable_path).await? {
        Some(ent) => {
            println!("{}", serde_json::to_string_pretty(&ent)?);
            Ok(())
        }
        None => bail!("entitlement does not have any entitlements or is not an executable"),
    }
}

async fn parse_entitlements_file<P: AsRef<Path>>(
    path: P,
) -> Result<Option<HashSet<IPSWExecutableEntitlements>>> {
    if !path_is_macho(&path)? {
        return Ok(None);
    }

    let mut macho_file = File::open(&path).await?;
    let mut macho_bin_data = Vec::new();
    macho_file.read_to_end(&mut macho_bin_data).await?;

    let macho = match MachOBinary::parse(&macho_bin_data) {
        Ok(macho) => macho,
        Err(e) => return Err(e.into()),
    };

    if !macho.is_executable() {
        return Ok(None);
    }

    // If executable is fat, only treat first ? / add two entries in database ?
    if let Some(code_signature) = macho.code_signature()? {
        if let Some(entitlements) = code_signature.entitlements()? {
            let plist_value = Value::from_reader(Cursor::new(entitlements.as_str()))?;

            return Ok(Some(parse_entitlement(plist_value)?));
        }
    }

    Ok(None)
}

fn parse_entitlement(value: Value) -> Result<HashSet<IPSWExecutableEntitlements>> {
    let mut entitlements = HashSet::new();
    match value {
        Value::Array(values) => {
            for value in values {
                entitlements = entitlements
                    .union(&parse_entitlement(value)?)
                    .cloned()
                    .collect();
            }
        }
        Value::Dictionary(dictionary) => {
            for (key, value) in dictionary {
                let key = key.to_owned();
                let sub_entitlements = parse_entitlement(value)?;
                for ent in sub_entitlements.into_iter() {
                    if ent.key.is_empty() {
                        // entitlement type with no key (bool, integer, real...)
                        entitlements.insert(IPSWExecutableEntitlements {
                            key: key.clone(),
                            value: ent.value,
                        });
                    } else {
                        entitlements.insert(IPSWExecutableEntitlements {
                            key: format!("{}.{}", key, ent.key),
                            value: ent.value,
                        });
                    }
                }
            }
        }
        Value::Boolean(value) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: value.to_string(),
            });
        }
        Value::Data(items) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: BASE64_STANDARD.encode(items),
            });
        }
        Value::Date(date) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: date.to_xml_format(),
            });
        }
        Value::Real(real) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: real.to_string(),
            });
        }
        Value::Integer(integer) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: integer.to_string(),
            });
        }
        Value::String(s) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: s.to_string(),
            });
        }
        Value::Uid(uid) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: uid.get().to_string(),
            });
        }
        _ => bail!("unknown value type"),
    }
    Ok(entitlements)
}
