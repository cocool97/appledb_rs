use serde::Serialize;

mod devices;
mod entitlements;
mod executable_entitlement;
mod executable_operating_system_version;
mod executables;
mod operating_system_versions;
mod operating_systems;

#[derive(Serialize)]
pub enum DBStatus {
    AlreadyExists(i32),
    Created(i32),
}

impl DBStatus {
    pub fn db_identifier(&self) -> i32 {
        match self {
            DBStatus::AlreadyExists(id) => *id,
            DBStatus::Created(id) => *id,
        }
    }
}
