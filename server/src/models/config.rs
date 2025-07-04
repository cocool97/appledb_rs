use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use std::{
    fmt::Display,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::fs::File;
use url::Url;

pub async fn read_configuration<P: AsRef<Path>>(path: P) -> Result<ServerConfig> {
    let file = File::open(&path).await.context(format!(
        "Cannot open configuration file at path {}...",
        path.as_ref().to_string_lossy()
    ))?;
    Ok(serde_norway::from_reader(file.into_std().await)?)
}

#[derive(Deserialize)]
pub struct ServerConfig {
    /// Server listen mode
    /// Can be http://127.0.0.1 or unix:/path/to/socket
    #[serde(deserialize_with = "deserialize_listen_mode")]
    pub listen_mode: ListenMode,
    /// Maximum HTTP body size
    pub http_max_body_size: usize,
    /// Path to database
    /// e.g:
    /// Postgres: postgres://username:pass@host/schema_name
    /// SQLite: sqlite://path/to/db.sqlite?mode=rwc
    pub database_url: String,
    /// Path to web sources
    pub web_sources_path: PathBuf,
    /// List of CORS domains to allow
    pub cors_allowed_origins: Option<Vec<String>>,
    /// Max concurrent tasks that can run concurrently
    pub max_concurrent_tasks: usize,
    /// Serve openapi documentation ?
    pub serve_openapi: bool,
}

#[derive(Deserialize)]
pub enum ListenMode {
    SocketAddr(SocketAddr),
    UnixSocket(PathBuf),
}

impl FromStr for ListenMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;

        match url.scheme() {
            "http" => {
                let host = url.host_str().ok_or_else(|| anyhow!("No host"))?;
                let port = url
                    .port_or_known_default()
                    .ok_or_else(|| anyhow!("No port"))?;
                Ok(Self::SocketAddr(SocketAddr::new(host.parse()?, port)))
            }
            "unix" => {
                let path = url.path();
                if path.is_empty() {
                    bail!("empty unix socket path specified...")
                }
                Ok(Self::UnixSocket(PathBuf::from(path)))
            }
            scheme => Err(anyhow!("Invalid scheme {scheme}")),
        }
    }
}

impl Display for ListenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListenMode::SocketAddr(socket_addr) => write!(f, "http://{socket_addr}",),
            ListenMode::UnixSocket(path) => write!(f, "unix:{}", path.display()),
        }
    }
}

fn deserialize_listen_mode<'de, D>(deserializer: D) -> Result<ListenMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    ListenMode::from_str(&s).map_err(serde::de::Error::custom)
}
