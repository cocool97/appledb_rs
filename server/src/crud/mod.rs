use sea_orm::FromQueryResult;
use serde::Serialize;

mod devices;
mod entitlements;
mod executable_entitlement;
mod executable_framework;
mod executable_operating_system_version;
mod executables;
mod frameworks;
mod operating_system_versions;
mod operating_systems;

use utoipa::ToSchema;

#[derive(Serialize)]
pub enum DBStatus {
    AlreadyExists(i64),
    Created(i64),
}

#[derive(FromQueryResult, ToSchema, Serialize)]
pub struct OperatingSystemVersionExtended {
    pub id: i64,
    pub display_name: Option<String>,
    pub model_code: String,
    pub version: String,
}

impl DBStatus {
    pub fn db_identifier(&self) -> i64 {
        match self {
            DBStatus::AlreadyExists(id) => *id,
            DBStatus::Created(id) => *id,
        }
    }
}
