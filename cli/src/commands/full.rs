use std::{path::PathBuf, str::FromStr};

use crate::data_writers::DataWriter;
use crate::models::ParseSubcommand;
use crate::parsers::{FrameworksParser, IPSWParser};
use crate::utils::parse_macho;
use crate::{ipsw_executables::IPSWExecutablesIterator, parsers::EntitlementsParser};
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn parse_full_subcommand(
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

    let mut frameworks_parser = FrameworksParser::new(platform.into(), &model_code, &version);

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

                if let Err(e) = frameworks_parser
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
    frameworks_parser.post_results(data_writer).await?;

    Ok(())
}
