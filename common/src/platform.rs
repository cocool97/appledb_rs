use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Platform {
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "macos")]
    MacOS,
    #[serde(rename = "watchos")]
    WatchOS,
    #[serde(rename = "tvos")]
    TvOS,
}

impl Platform {
    pub fn from_product_name(product_name: &str) -> Option<Self> {
        match product_name {
            "iPhone OS" => Some(Self::Ios),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Platform::Ios => "ios",
            Platform::MacOS => "macos",
            Platform::WatchOS => "watchos",
            Platform::TvOS => "tvos",
        }
    }
}
