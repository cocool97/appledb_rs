use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ServerStats {
    pub known_devices: u64,
    pub known_operating_system_versions: u64,
    pub known_executables: usize,
    pub known_entitlements: u64,
    pub known_frameworks: usize,
}
