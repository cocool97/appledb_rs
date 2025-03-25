use std::sync::Arc;
use strum::EnumCount;

use crate::handlers::public::{
    devices::{
        __path_get_device_operating_system_versions, __path_get_devices,
        get_device_operating_system_versions, get_devices,
    },
    entitlements::{
        __path_diff_entitlements_for_executables, __path_get_entitlements,
        __path_get_entitlements_by_id, __path_get_entitlements_by_name,
        diff_entitlements_for_executables, get_entitlements_by_name,
    },
    executables::{
        __path_get_all_executables_entitlements, __path_get_executable_entitlements,
        __path_get_executables, __path_get_executables_by_id, __path_get_executables_by_name,
        __path_get_executables_with_entitlement_for_os_version, get_all_executables_entitlements,
        get_executable_entitlements, get_executables_by_name,
    },
    operating_system_versions::{
        __path_get_operating_system_versions, __path_get_operating_system_versions_by_id,
    },
    operating_systems::{__path_get_operating_system_by_id, __path_get_operating_systems},
    stats::{__path_get_stats, get_stats},
};
use appledb_common::routes::PublicRoutes;
use axum::{Router, routing::get};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::models::AppState;

use super::{
    entitlements::{get_entitlements, get_entitlements_by_id},
    executables::{
        get_executables, get_executables_by_id, get_executables_with_entitlement_for_os_version,
    },
    operating_system_versions::{
        get_operating_system_versions, get_operating_system_versions_by_id,
    },
    operating_systems::{get_operating_system_by_id, get_operating_systems},
};

pub fn get_public_router() -> Router<Arc<AppState>> {
    #[derive(OpenApi)]
    #[openapi(paths(
        get_stats,
        get_operating_systems,
        get_operating_system_by_id,
        get_devices,
        get_device_operating_system_versions,
        get_operating_system_versions,
        get_operating_system_versions_by_id,
        get_executables,
        get_executables_by_id,
        get_executables_by_name,
        get_executables_with_entitlement_for_os_version,
        get_all_executables_entitlements,
        get_executable_entitlements,
        get_entitlements,
        get_entitlements_by_id,
        get_entitlements_by_name,
        diff_entitlements_for_executables
    ))]
    struct ApiDoc;

    // Update each path to add PUBLIC_ROUTES prefix
    let mut openapi = ApiDoc::openapi();
    openapi.paths.paths = openapi
        .paths
        .paths
        .iter_mut()
        .map(|(path, item)| {
            (
                format!("{}{}", PublicRoutes::route_prefix(), path),
                item.to_owned(),
            )
        })
        .collect();

    // Check at compile time that every registered endpoint is documented
    assert_eq!(
        openapi.paths.paths.len(),
        PublicRoutes::COUNT,
        "all public handlers aren't documented..."
    );

    Router::new()
        .merge(
            SwaggerUi::new("/swagger")
                .config(Config::from(
                    PublicRoutes::route_prefix().to_owned() + "/openapi.json",
                ))
                .url("/openapi.json", openapi),
        )
        .route(PublicRoutes::GetStats.to_string().as_str(), get(get_stats))
        // ##################
        // Operating systems
        // ##################
        .route(
            PublicRoutes::GetOperatingSystems.to_string().as_str(),
            get(get_operating_systems),
        )
        .route(
            PublicRoutes::GetOperatingSystemById.to_string().as_str(),
            get(get_operating_system_by_id),
        )
        // ##################
        // Devices
        // ##################
        .route(
            PublicRoutes::GetDevices.to_string().as_str(),
            get(get_devices),
        )
        .route(
            PublicRoutes::GetDeviceVersions.to_string().as_str(),
            get(get_device_operating_system_versions),
        )
        // ##################
        // Operating system versions
        // ##################
        .route(
            PublicRoutes::GetOperatingSystemVersions
                .to_string()
                .as_str(),
            get(get_operating_system_versions),
        )
        .route(
            PublicRoutes::GetOperatingSystemVersionsById
                .to_string()
                .as_str(),
            get(get_operating_system_versions_by_id),
        )
        // ##################
        // Executables
        // ##################
        .route(
            PublicRoutes::GetExecutables.to_string().as_str(),
            get(get_executables),
        )
        .route(
            PublicRoutes::GetExecutablesById.to_string().as_str(),
            get(get_executables_by_id),
        )
        .route(
            PublicRoutes::GetExecutablesByName.to_string().as_str(),
            get(get_executables_by_name),
        )
        .route(
            PublicRoutes::GetExecutablesWithEntitlement
                .to_string()
                .as_str(),
            get(get_executables_with_entitlement_for_os_version),
        )
        .route(
            PublicRoutes::GetAllExecutablesEntitlements
                .to_string()
                .as_str(),
            get(get_all_executables_entitlements),
        )
        // ##################
        // Entitlements
        // ##################
        .route(
            PublicRoutes::GetEntitlements.to_string().as_str(),
            get(get_entitlements),
        )
        .route(
            PublicRoutes::GetEntitlementsById.to_string().as_str(),
            get(get_entitlements_by_id),
        )
        .route(
            PublicRoutes::GetEntitlementsByName.to_string().as_str(),
            get(get_entitlements_by_name),
        )
        .route(
            PublicRoutes::GetExecutableEntitlements.to_string().as_str(),
            get(get_executable_entitlements),
        )
        .route(
            PublicRoutes::GetDiffEntitlementsExecutables
                .to_string()
                .as_str(),
            get(diff_entitlements_for_executables),
        )
}
