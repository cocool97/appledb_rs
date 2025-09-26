use std::{path::PathBuf, str::FromStr};

use crate::{
    ipsw_executables::IPSWExecutablesIterator,
    models::FrameworksSubcommands,
    parsers::{FrameworksParser, IPSWParser},
    server_controller::ServerController,
    utils::parse_macho,
};
use anyhow::Result;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn parse_framework_subcommand(
    server_url: String,
    insecure: bool,
    subcommand: FrameworksSubcommands,
) -> Result<()> {
    match subcommand {
        FrameworksSubcommands::Parse {
            mount_point,
            platform,
            version,
            model_code,
        } => {
            log::info!(
                "IPSW has platform={platform}, model_code={model_code} and version={version}"
            );

            let mut frameworks_parser =
                FrameworksParser::new(platform.into(), &model_code, &version);

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
                    Ok(None) => {
                        continue;
                    }
                    Err(e) => log::error!("error while parsing macho {}: {e}", entry.display()),
                }
            }

            let server_controller = ServerController::new(server_url, insecure)?;

            let frameworks_task_uuid = frameworks_parser.post_results(&server_controller).await?;

            log::info!("Received frameworks task UUID: {frameworks_task_uuid}");

            Ok(())
        }
    }
}
