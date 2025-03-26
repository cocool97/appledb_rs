mod commands;
mod models;
mod server_controller;
mod utils;

use anyhow::Result;
use appledb_common::config::read_configuration;
use clap::Parser;
use commands::{parse_entitlements_command, parse_os_subcommand};
use models::{Opts, OptsSubCommands};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    utils::set_logger(opts.debug)?;

    let configuration = read_configuration(opts.config_path).await?;

    let res = match opts.command {
        OptsSubCommands::Ent(ent_sub_commands) => {
            parse_entitlements_command(configuration, ent_sub_commands).await
        }
        OptsSubCommands::OperatingSystem(operating_systems_subcommands) => {
            parse_os_subcommand(configuration, operating_systems_subcommands).await
        }
    };

    if let Err(e) = res {
        log::error!("{}", e);
    }

    Ok(())
}
