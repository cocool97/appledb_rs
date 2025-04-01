use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    /// Path to server configuration file
    #[clap(short = 'c', long = "config", env = "CONFIG_PATH")]
    pub config_path: PathBuf,
}
