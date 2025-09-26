mod commands;
mod ipsw_executables;
mod models;
mod parsers;
mod server_controller;
mod utils;

use anyhow::Result;
use clap::Parser;
use commands::{
    parse_entitlements_command, parse_framework_subcommand, parse_full_subcommand,
    parse_os_subcommand, parse_tasks_command,
};
use models::{Opts, OptsSubCommands};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    utils::set_logger(opts.debug)?;

    let res = match opts.command {
        OptsSubCommands::Ent(ent_sub_commands) => {
            parse_entitlements_command(opts.server_url, opts.insecure, ent_sub_commands).await
        }
        OptsSubCommands::OperatingSystem(operating_systems_subcommands) => {
            parse_os_subcommand(
                opts.server_url,
                opts.insecure,
                operating_systems_subcommands,
            )
            .await
        }
        OptsSubCommands::Tasks(tasks_subcommands) => {
            parse_tasks_command(opts.server_url, opts.insecure, tasks_subcommands).await
        }
        OptsSubCommands::Frameworks(frameworks_subcommands) => {
            parse_framework_subcommand(opts.server_url, opts.insecure, frameworks_subcommands).await
        }
        OptsSubCommands::Full(full_subcommand) => {
            parse_full_subcommand(opts.server_url, opts.insecure, full_subcommand).await
        }
    };

    if let Err(e) = res {
        log::error!("{e}");
    }

    Ok(())
}
