use chrono::Local;
use std::time::Duration;

use anyhow::Result;

use crate::{models::TasksSubcommands, server_controller::ServerController};

pub async fn parse_tasks_command(server_url: String, subcommand: TasksSubcommands) -> Result<()> {
    match subcommand {
        TasksSubcommands::Follow { interval } => follow_tasks(server_url, interval).await,
    }
}

async fn follow_tasks(server_url: String, interval: u64) -> Result<()> {
    let server_controller = ServerController::new(server_url)?;

    loop {
        let running_tasks = server_controller.get_running_tasks().await?;

        let now = Local::now();
        println!("### ~ {} ~ ###", now.to_rfc2822());
        if running_tasks.is_empty() {
            println!("\tNo task running...")
        } else {
            for (task_uuid, progress) in running_tasks {
                println!(
                    "\tUUID: {} - Progress: {}/{} ({}%)",
                    task_uuid,
                    progress.done,
                    progress.total,
                    (progress.done as f32) / (progress.total as f32) * 100.0 // dividing by 0 does not panic with f32
                )
            }
        }

        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}
