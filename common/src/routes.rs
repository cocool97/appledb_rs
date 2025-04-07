use std::fmt::Display;

use strum_macros::EnumCount;

// Network "base" routes
pub const ADMIN_ROUTES: &str = "/api/admin";

// ##################
// Admin-specific routes (authentication required)
// ##################

// Operating system versions
pub const POST_OPERATING_SYSTEM_VERSION: &str = "/operating_system_version";

// Executables
pub const POST_EXECUTABLE: &str = "/executable";

// Entitlements
pub const POST_EXECUTABLE_ENTITLEMENTS_ROUTE: &str = "/executable/entitlements";

// ##################
// Public routes
// ##################

// Operating systems
#[derive(EnumCount)]
pub enum PublicRoutes {
    // Get stats about server
    GetStats,

    // Operating systems
    GetOperatingSystems,
    GetOperatingSystemById,

    // Devices
    GetDevices,
    GetDeviceVersions,

    // Operating system versions
    GetOperatingSystemVersions,
    GetOperatingSystemVersionsById,
    GetOperatingSystemVersionsExtended,

    // Executables
    GetExecutableVersions,
    GetExecutableEntitlements,
    GetAllExecutables,
    GetAllExecutablesEntitlements,
    GetDiffExecutablesOperatingSystemVersion,

    // Entitlements
    GetDiffEntitlementsExecutables,
}

impl PublicRoutes {
    pub fn route_prefix() -> &'static str {
        "/api/v1"
    }
}

impl From<PublicRoutes> for String {
    fn from(value: PublicRoutes) -> Self {
        String::from(&value)
    }
}

impl From<&PublicRoutes> for String {
    fn from(value: &PublicRoutes) -> Self {
        match value {
            PublicRoutes::GetStats => "/stats".to_string(),
            PublicRoutes::GetOperatingSystems => "/operating_systems/all".to_string(),
            PublicRoutes::GetOperatingSystemById => "/operating_systems/{id}".to_string(),
            PublicRoutes::GetAllExecutablesEntitlements => {
                "/operating_systems/{id}/executable_entitlements".to_string()
            }
            PublicRoutes::GetAllExecutables => "/executables/all".to_string(),
            PublicRoutes::GetDevices => "/devices/all".to_string(),
            PublicRoutes::GetDeviceVersions => {
                "/devices/{id}/operating_system_versions".to_string()
            }
            PublicRoutes::GetOperatingSystemVersions => {
                "/operating_system_versions/all".to_string()
            }
            PublicRoutes::GetOperatingSystemVersionsById => {
                "/operating_system_versions/{id}".to_string()
            }
            PublicRoutes::GetOperatingSystemVersionsExtended => {
                "/operating_system_versions/extended".to_string()
            }
            PublicRoutes::GetExecutableVersions => "/executables/{id}/versions".to_string(),
            PublicRoutes::GetExecutableEntitlements => "/executable/{id}/entitlements".to_string(),
            PublicRoutes::GetDiffExecutablesOperatingSystemVersion => {
                "/executables/diff/{from_executable_id}/{to_executable_id}".to_string()
            }
            PublicRoutes::GetDiffEntitlementsExecutables => {
                "/entitlements/diff/{from_executable_id}/{to_executable_id}".to_string()
            }
        }
    }
}

impl Display for PublicRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
