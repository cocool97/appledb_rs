use serde::{Deserialize, Serialize};
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

    pub fn name(&self) -> &'static str {
        match self {
            Platform::Ios => "ios",
            Platform::IpadOS => "ipados",
            Platform::MacOS => "macos",
            Platform::VisionOS => "visionos",
        }
    }
}
