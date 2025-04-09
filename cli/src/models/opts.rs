use appledb_common::Platform;
use clap::{Parser, Subcommand, ValueEnum};
use std::{fmt::Display, path::PathBuf};

#[derive(Parser)]
pub struct Opts {
    /// Path to configuration file
    #[clap(short = 's', long = "server-url")]
    pub server_url: String,
    #[clap(long = "debug")]
    pub debug: bool,
    #[clap(subcommand)]
    pub command: OptsSubCommands,
}

#[derive(Subcommand)]
pub enum OptsSubCommands {
    /// Entitlement related subcommands
    #[clap(subcommand)]
    Ent(EntSubCommands),
    /// Operating system related subcommands
    #[clap(subcommand)]
    OperatingSystem(OperatingSystemsSubcommands),
}

#[derive(Subcommand)]
pub enum EntSubCommands {
    /// Parse and send entitlements of all mach-o executables in a directory.
    Parse {
        /// Path to local mount point where ipsw is already mounted
        #[clap(short = 'd', long = "mount-point")]
        mount_point: PathBuf,
        /// Platform from which this IPSW mount is originated
        #[clap(short = 'p', long = "platform")]
        platform: OptsPlatform,
        /// Version from which this IPSW is originated
        #[clap(short = 'v', long = "version")]
        version: String,
        /// Device model (under iPhone17,5 - iPad15,5)...
        #[clap(short = 'm', long = "model_code")]
        model_code: String,
    },
    /// Dump entitlements from a Mach-o executable
    DumpEnt {
        /// Mach-O executable path
        #[clap(short = 'b', long = "bin")]
        executable_path: PathBuf,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OptsPlatform {
    Ios,
    MacOS,
    WatchOS,
    TvOS,
}

impl Display for OptsPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptsPlatform::Ios => write!(f, "ios"),
            OptsPlatform::MacOS => write!(f, "macos"),
            OptsPlatform::WatchOS => write!(f, "watchos"),
            OptsPlatform::TvOS => write!(f, "tvos"),
        }
    }
}

impl From<OptsPlatform> for Platform {
    fn from(value: OptsPlatform) -> Self {
        match value {
            OptsPlatform::Ios => Self::Ios,
            OptsPlatform::MacOS => Self::MacOS,
            OptsPlatform::WatchOS => Self::WatchOS,
            OptsPlatform::TvOS => Self::TvOS,
        }
    }
}

impl From<Platform> for OptsPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Ios => Self::Ios,
            Platform::MacOS => Self::MacOS,
            Platform::WatchOS => Self::WatchOS,
            Platform::TvOS => Self::TvOS,
        }
    }
}

#[derive(Subcommand)]
pub enum OperatingSystemsSubcommands {
    /// List known operating systems
    List {},
}
