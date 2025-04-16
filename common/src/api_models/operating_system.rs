use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct ExtendedOperatingSystemVersions {
    pub id: i64,
    pub version: String,
    pub model_code: String,
    pub display_name: Option<String>,
}

impl
    From<(
        entity::operating_system_version::Model,
        entity::device::Model,
    )> for ExtendedOperatingSystemVersions
{
    fn from(
        (os_version, device): (
            entity::operating_system_version::Model,
            entity::device::Model,
        ),
    ) -> Self {
        ExtendedOperatingSystemVersions {
            id: os_version.id,
            version: os_version.version,
            model_code: device.model_code,
            display_name: device.display_name,
        }
    }
}
