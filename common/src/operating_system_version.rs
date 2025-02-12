use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOperatingSystemVersion {
    pub operating_system_id: i32,
    pub version: String,
}
