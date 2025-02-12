use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateExecutable {
    pub name: String,
    pub operating_system_version_id: i32,
}
