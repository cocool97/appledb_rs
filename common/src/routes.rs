use std::fmt::Display;

use strum_macros::EnumCount;

// ##################
// Admin-specific routes (authentication required)
// ##################
#[derive(EnumCount)]
pub enum AdminRoutes {
    StopRunningTask,
    PostExecutableEntitlements,
    PostExecutableFrameworks,
}

impl AdminRoutes {
    pub fn route_prefix() -> &'static str {
        "/api/admin"
    }
}

impl From<AdminRoutes> for String {
    fn from(value: AdminRoutes) -> Self {
        String::from(&value)
    }
}

impl From<&AdminRoutes> for String {
    fn from(value: &AdminRoutes) -> Self {
        match value {
            AdminRoutes::StopRunningTask => "/tasks/{task_id}/stop".to_string(),
            AdminRoutes::PostExecutableEntitlements => "/executable/entitlements".to_string(),
            AdminRoutes::PostExecutableFrameworks => "/executable/frameworks".to_string(),
        }
    }
}

impl Display for AdminRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

// ##################
// Public routes
// ##################
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
    GetOperatingSystemVersionsExecutables,
    GetOperatingSystemVersionsFrameworks,

    // Executables
    GetExecutableVersions,
    GetExecutableEntitlements,
    GetAllExecutables,
    GetAllExecutablesEntitlements,
    GetDiffExecutablesOperatingSystemVersion,

    // Entitlements
    GetDiffEntitlementsExecutables,

    // Frameworks
    GetDiffFrameworksExecutables,
    GetExecutableFrameworks,
    GetAllFrameworks,
    GetFrameworkVersions,
    GetFrameworkExecutables,

    // Tasks
    GetRunningTasks,
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
            PublicRoutes::GetAllExecutablesEntitlements =>"/operating_systems/{id}/executable_entitlements".to_string(),
            PublicRoutes::GetAllExecutables => "/executables/all".to_string(),
            PublicRoutes::GetDevices => "/devices/all".to_string(),
            PublicRoutes::GetDeviceVersions => "/devices/{id}/operating_system_versions".to_string(),
            PublicRoutes::GetOperatingSystemVersions => "/operating_system_versions/all".to_string(),
            PublicRoutes::GetOperatingSystemVersionsById => "/operating_system_versions/{id}".to_string(),
            PublicRoutes::GetOperatingSystemVersionsExtended => "/operating_system_versions/extended".to_string(),
            PublicRoutes::GetOperatingSystemVersionsExecutables => "/operating_system_versions/{operating_system_version_id}/executables".to_string(),
            PublicRoutes::GetOperatingSystemVersionsFrameworks => "/operating_system_versions/{operating_system_version_id}/frameworks".to_string(),
            PublicRoutes::GetExecutableVersions => "/executables/{id}/versions".to_string(),
            PublicRoutes::GetExecutableEntitlements => "/executable/{id}/entitlements".to_string(),
            PublicRoutes::GetDiffExecutablesOperatingSystemVersion => "/executables/diff/{from_operating_system_version_id}/{to_operating_system_version_id}".to_string(),
            PublicRoutes::GetDiffEntitlementsExecutables => "/entitlements/diff/{from_executable_id}/{to_executable_id}".to_string(),
            PublicRoutes::GetDiffFrameworksExecutables => "/frameworks/diff/{from_executable_id}/{to_executable_id}".to_string(),
            PublicRoutes::GetAllFrameworks => "/frameworks/all".to_string(),
            PublicRoutes::GetFrameworkVersions => "/frameworks/{id}/versions".to_string(),
            PublicRoutes::GetFrameworkExecutables => "/frameworks/{framework_id}/executables/{operating_system_version_id}".to_string(),
            PublicRoutes::GetExecutableFrameworks => "/executable/{executable_operating_system_id}/frameworks".to_string(),
            PublicRoutes::GetRunningTasks => "/tasks/running".to_string(),
        }
    }
}

impl Display for PublicRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
