use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct OperatingSystem {
    pub id: i64,
    pub name: OperatingSystemName,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub enum OperatingSystemName {
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "ipados")]
    Ipados,
    #[serde(rename = "macos")]
    MacOS,
    #[serde(rename = "visionos")]
    VisionOS,
    #[serde(rename = "tvos")]
    TvOS,
}

impl TryFrom<entity::operating_system::Model> for OperatingSystem {
    type Error = anyhow::Error;

    fn try_from(value: entity::operating_system::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: OperatingSystemName::try_from(value.name.as_str())?,
        })
    }
}

impl TryFrom<&str> for OperatingSystemName {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ios" => Ok(Self::Ios),
            "ipados" => Ok(Self::Ipados),
            "macos" => Ok(Self::MacOS),
            "visionos" => Ok(Self::VisionOS),
            "tvos" => Ok(Self::TvOS),
            v => bail!("unknown operating system value {v}..."),
        }
    }
}
