use crate::{models::OperatingSystemsSubcommands, server_controller::ServerController};
use anyhow::Result;

pub async fn parse_os_subcommand(
    server_url: String,
    subcommand: OperatingSystemsSubcommands,
) -> Result<()> {
    match subcommand {
        OperatingSystemsSubcommands::List {} => {
            let server_controller = ServerController::new(server_url)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&server_controller.get_operating_systems().await?)?
            );
            Ok(())
        }
    }
}
