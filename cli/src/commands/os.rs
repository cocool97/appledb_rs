use crate::{models::OperatingSystemsSubcommands, server_controller::ServerController};
use anyhow::Result;
use appledb_common::config::ServerConfig;

pub async fn parse_os_subcommand(
    configuration: ServerConfig,
    subcommand: OperatingSystemsSubcommands,
) -> Result<()> {
    match subcommand {
        OperatingSystemsSubcommands::List {} => {
            let server_controller = ServerController::new(configuration.listen_mode)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&server_controller.get_operating_systems().await?)?
            );
            Ok(())
        }
    }
}
