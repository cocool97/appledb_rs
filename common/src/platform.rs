use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ToSchema)]
pub enum Platform {
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "ipados")]
    IpadOS,
    #[serde(rename = "macos")]
    MacOS,
    #[serde(rename = "visionos")]
    VisionOS,
}

impl Platform {
    pub fn from_product_name(product_name: &str) -> Option<Self> {
        match product_name {
            "iPhone OS" => Some(Self::Ios),
            _ => None,
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Platform::Ios => "ios",
            Platform::IpadOS => "ipados",
            Platform::MacOS => "macos",
            Platform::VisionOS => "visionos",
        };
        write!(f, "{name}")
    }
}
