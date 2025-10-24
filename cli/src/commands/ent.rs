use anyhow::Result;
use std::{path::PathBuf, str::FromStr};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    data_writers::DataWriter,
    ipsw_executables::IPSWExecutablesIterator,
    models::ParseSubcommand,
    parsers::{EntitlementsParser, IPSWParser},
    utils::parse_macho,
};

pub async fn parse_entitlements_command(
    data_writer: &dyn DataWriter,
    parse_subcommand: ParseSubcommand,
) -> Result<()> {
    let platform = parse_subcommand.platform;
    let model_code = parse_subcommand.model_code;
    let mount_point = parse_subcommand.mount_point;
    let version = parse_subcommand.version;

    log::info!("IPSW has platform={platform}, model_code={model_code} and version={version}");

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
            Ok(None) => {}
            Err(e) => log::error!("error while parsing macho {}: {e}", entry.display()),
        }
    }

    entitlements_parser.post_results(data_writer).await?;

    Ok(())
}
