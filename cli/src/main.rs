mod commands;
mod data_writers;
mod ipsw_executables;
mod models;
mod parsers;
mod utils;

use anyhow::Result;
use clap::Parser;
use commands::{
    parse_entitlements_command, parse_framework_subcommand, parse_full_subcommand,
    parse_tasks_command,
};
use models::{AppleDBSubcommand, Opts};

use crate::models::ParsingType;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    utils::set_logger(opts.debug);

    let res = match opts.command {
        AppleDBSubcommand::Parse {
            parsing_type,
            output,
            command,
        } => {
            let data_writer =
                utils::data_writer_from_ops(opts.server_url, opts.insecure, output).await?;

            match parsing_type {
                ParsingType::Full => parse_full_subcommand(data_writer.as_ref(), command).await,
                ParsingType::Ent => parse_entitlements_command(data_writer.as_ref(), command).await,
                ParsingType::Frameworks => {
                    parse_framework_subcommand(data_writer.as_ref(), command).await
                }
            }
        }
        AppleDBSubcommand::Tasks { command } => {
            parse_tasks_command(opts.server_url, opts.insecure, command).await
        }
    };

    if let Err(e) = res {
        log::error!("{e}");
    }

    Ok(())
}
