use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{models::TasksSubcommands, server_controller::ServerController};

pub async fn parse_tasks_command(
    server_url: String,
    insecure: bool,
    subcommand: TasksSubcommands,
) -> Result<()> {
    match subcommand {
        TasksSubcommands::Follow { interval } => follow_tasks(server_url, insecure, interval).await,
    }
}

async fn follow_tasks(server_url: String, insecure: bool, interval: u64) -> Result<()> {
    let server_controller = ServerController::new(server_url, insecure)?;
    let multi_progress = MultiProgress::new();

    let mut bars: HashMap<String, ProgressBar> = HashMap::new();
    let mut start_times: HashMap<String, DateTime<Utc>> = HashMap::new();

    let pb_style = ProgressStyle::with_template("{msg} [{wide_bar}] {pos}/{len} ({percent}%)")
        .unwrap()
        .progress_chars("##-");

    loop {
        let running_tasks = server_controller.get_running_tasks().await?;

        let current_ids: HashSet<_> = running_tasks.keys().cloned().collect();

        let known_ids: Vec<String> = bars.keys().cloned().collect();
        for task_id in known_ids {
            if !current_ids.contains(&task_id) {
                if let Some(pb) = bars.remove(&task_id) {
                    pb.finish_and_clear();
                }
                start_times.remove(&task_id);
            }
        }

        for (task_uuid, progress) in &running_tasks {
            let pb = bars.entry(task_uuid.clone()).or_insert_with(|| {
                let pb = multi_progress.add(ProgressBar::new(progress.total()));
                pb.set_style(pb_style.clone());
                pb
            });

            start_times
                .entry(task_uuid.clone())
                .or_insert_with(Utc::now);
            let elapsed = format_duration(*start_times.get(task_uuid).unwrap());
            let msg = format!("{task_uuid} | {elapsed}");

            pb.set_message(msg);
            pb.set_position(progress.done());
        }

        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

fn format_duration(start_time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(start_time);
    let mins = duration.num_minutes();
    let secs = duration.num_seconds() % 60;
    format!("{mins:02}m{secs:02}s")
}
