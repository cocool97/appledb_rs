use anyhow::Result;
use std::{path::PathBuf, str::FromStr};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    ipsw_executables::IPSWExecutablesIterator,
    models::EntSubCommands,
    parsers::{EntitlementsParser, IPSWParser},
    server_controller::ServerController,
    utils::parse_macho,
};

pub async fn parse_entitlements_command(
    server_url: String,
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
            let mut entitlements_parser =
                EntitlementsParser::new(platform.clone().into(), &model_code, &version);

            for entry in IPSWExecutablesIterator::new(&mount_point).flatten() {
                let stripped_path = entry.strip_prefix(&mount_point)?;
                let full_absolute_path = match &stripped_path.is_absolute() {
                    true => stripped_path.to_path_buf(),
                    false => PathBuf::from_str("/")?.join(stripped_path),
                };

                let mut macho_file = File::open(&entry).await?;
                let mut macho_bin_data = Vec::new();
                macho_file.read_to_end(&mut macho_bin_data).await?;

                match parse_macho(&macho_bin_data) {
                    Ok(Some(macho)) => {
                        if let Err(e) = entitlements_parser
                            .parse_file(&full_absolute_path, &macho)
                            .await
                        {
                            log::error!(
                                "got error while parsing file {}: {e}",
                                full_absolute_path.display()
                            );
                        }
                    }
                    Ok(None) => {
                        continue;
                    }
                    Err(e) => log::error!("error while parsing macho {}: {e}", entry.display()),
                }
            }

            let server_controller = ServerController::new(server_url)?;

            let entitlements_task_uuid =
                entitlements_parser.post_results(&server_controller).await?;

            log::info!("Received entitlements task UUID: {entitlements_task_uuid}",);

            Ok(())
        }
    }
}
