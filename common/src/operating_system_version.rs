use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOperatingSystemVersion {
    pub device_id: i32,
    pub operating_system_id: i32,
    pub version: String,
}
