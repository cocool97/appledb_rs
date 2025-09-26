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
    /// Allow insecure TLS connections. Be careful when using this option!
    #[clap(long = "insecure")]
    pub insecure: bool,
}

#[derive(Subcommand)]
pub enum OptsSubCommands {
    /// Entitlement related subcommands
    #[clap(subcommand)]
    Ent(EntSubCommands),
    /// Operating system related subcommands
    #[clap(subcommand)]
    OperatingSystem(OperatingSystemsSubcommands),
    /// Get running tasks information
    #[clap(subcommand)]
    Tasks(TasksSubcommands),
    /// Parse executables linked frameworks
    #[clap(subcommand)]
    Frameworks(FrameworksSubcommands),
    /// Parse everything !
    #[clap(subcommand)]
    Full(FullSubcommand),
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
pub enum OperatingSystemsSubcommands {
    /// List known operating systems
    List {},
}

#[derive(Subcommand)]
pub enum TasksSubcommands {
    /// Follow running tasks
    Follow {
        /// Polling interval
        #[clap(long = "interval", default_value_t = 5)]
        interval: u64,
    },
}

#[derive(Subcommand)]
pub enum FrameworksSubcommands {
    /// Parse frameworks
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
        /// Device model (e.g iPhone17,5 - iPad15,5 ...)
        #[clap(short = 'm', long = "model_code")]
        model_code: String,
    },
}

#[derive(Subcommand)]
pub enum FullSubcommand {
    /// Parse and send everything related to given IPSW.
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
}
