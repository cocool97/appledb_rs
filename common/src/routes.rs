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
    // Operating systems
    GetOperatingSystems,
    GetOperatingSystemById,

    // Operating system versions
    GetOperatingSystemVersions,
    GetOperatingSystemVersionsById,

    // Executables
    GetExecutables,
    GetExecutablesById,
    GetExecutablesByName,
    GetExecutablesWithEntitlement,

    // Entitlements
    GetEntitlements,
    GetEntitlementsById,
    GetEntitlementsByName,
    GetEntitlementsForExecutable,
    DiffEntitlementsExecutables,
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
            PublicRoutes::GetOperatingSystems => "/operating_systems/get".to_string(),
            PublicRoutes::GetOperatingSystemById => "/operating_systems/get/{id}".to_string(),
            PublicRoutes::GetOperatingSystemVersions => {
                "/operating_system_versions/get".to_string()
            }
            PublicRoutes::GetOperatingSystemVersionsById => {
                "/operating_system_versions/get/{id}".to_string()
            }
            PublicRoutes::GetExecutables => "/executables/get".to_string(),
            PublicRoutes::GetExecutablesById => "/executables/get/{id}".to_string(),
            PublicRoutes::GetExecutablesByName => "/executables/get_by_name/{name}".to_string(),
            PublicRoutes::GetExecutablesWithEntitlement => {
                "/executables/get/{operating_system_version_id}/{entitlement_key}".to_string()
            }
            PublicRoutes::GetEntitlements => "/entitlements/get".to_string(),
            PublicRoutes::GetEntitlementsById => "/entitlements/get/{id}".to_string(),
            PublicRoutes::GetEntitlementsByName => "/entitlements/get_by_name/{name}".to_string(),
            PublicRoutes::GetEntitlementsForExecutable => {
                "/entitlements/executable/get/{id}".to_string()
            }
            PublicRoutes::DiffEntitlementsExecutables => {
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
