use serde::Serialize;
use utoipa::ToSchema;

use crate::db_models::OperatingSystemVersion;

#[derive(Serialize, PartialEq, Eq, Hash, Clone, ToSchema)]
pub struct Device {
    pub id: i64,
    pub model_code: String,
    pub display_name: Option<String>,
    pub versions: Vec<OperatingSystemVersion>,
}

impl
    From<(
        entity::device::Model,
        Vec<entity::operating_system_version::Model>,
    )> for Device
{
    fn from(
        (device, operating_system_versions): (
            entity::device::Model,
            Vec<entity::operating_system_version::Model>,
        ),
    ) -> Self {
        Device {
            id: device.id,
            model_code: device.model_code,
            display_name: device.display_name,
            versions: operating_system_versions
                .into_iter()
                .map(OperatingSystemVersion::from)
                .collect(),
        }
    }
}
