use appledb_common::Platform;
use clap::{Parser, Subcommand, ValueEnum};
use std::{fmt::Display, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Opts {
    /// Set default log level to DEBUG
    #[clap(long = "debug")]
    pub debug: bool,
    /// Path to configuration file
    #[clap(short = 's', long = "server-url")]
    pub server_url: Option<String>,
    #[clap(subcommand)]
    pub command: AppleDBSubcommand,
    /// Allow insecure TLS connections. Be careful when using this option!
    #[clap(long = "insecure")]
    pub insecure: bool,
}

#[derive(Subcommand)]
pub enum AppleDBSubcommand {
    /// Parse data from all mach-o executables in a directory.
    Parse {
        /// Type of parsing desired
        parsing_type: ParsingType,
        /// Optional output directory
        #[clap(short = 'o', long = "output")]
        output: Option<PathBuf>,
        #[clap(flatten)]
        command: ParseSubcommand,
    },
    /// Interact with server-side tasks
    Tasks {
        #[clap(subcommand)]
        command: TasksSubcommands,
    },
}

#[derive(Clone, ValueEnum)]
pub enum ParsingType {
    Full,
    Ent,
    Frameworks,
}

#[derive(Clone, ValueEnum)]
pub enum OptsPlatform {
    Ios,
    IpadOS,
    MacOS,
    VisionOS,
}

impl Display for OptsPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptsPlatform::Ios => write!(f, "ios"),
            OptsPlatform::IpadOS => write!(f, "ipados"),
            OptsPlatform::MacOS => write!(f, "macos"),
            OptsPlatform::VisionOS => write!(f, "visionos"),
        }
    }
}

impl From<OptsPlatform> for Platform {
    fn from(value: OptsPlatform) -> Self {
        match value {
            OptsPlatform::Ios => Self::Ios,
            OptsPlatform::IpadOS => Self::IpadOS,
            OptsPlatform::MacOS => Self::MacOS,
            OptsPlatform::VisionOS => Self::VisionOS,
        }
    }
}

impl From<Platform> for OptsPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Ios => Self::Ios,
            Platform::IpadOS => Self::IpadOS,
            Platform::MacOS => Self::MacOS,
            Platform::VisionOS => Self::VisionOS,
        }
    }
}

#[derive(Subcommand)]
pub enum TasksSubcommands {
    Follow {
        /// Polling interval
        #[clap(long = "interval", default_value_t = 5)]
        interval: u64,
    },
}

#[derive(Parser)]
pub struct ParseSubcommand {
    /// Path to local mount point where ipsw is already mounted
    #[clap(short = 'd', long = "mount-point")]
    pub mount_point: PathBuf,
    /// Platform from which this IPSW mount is originated
    #[clap(short = 'p', long = "platform")]
    pub platform: OptsPlatform,
    /// Version from which this IPSW is originated
    #[clap(short = 'v', long = "version")]
    pub version: String,
    /// Device model (under iPhone17,5 - iPad15,5)...
    #[clap(short = 'm', long = "model_code")]
    pub model_code: String,
}
