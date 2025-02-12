use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct OperatingSystemVersion {
    pub id: i32,
    pub version: String,
    pub operating_system_id: i32,
}

impl From<entity::operating_system_version::Model> for OperatingSystemVersion {
    fn from(value: entity::operating_system_version::Model) -> Self {
        Self {
            id: value.id,
            version: value.version,
            operating_system_id: value.operating_system_id,
        }
    }
}
