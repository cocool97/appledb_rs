use appledb_common::routes::PUBLIC_ROUTES_PREFIX;
use std::sync::Arc;
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::public::{
    devices::{
        __path_get_device_operating_system_versions, __path_get_devices,
        get_device_operating_system_versions, get_devices,
    },
    entitlements::{__path_diff_entitlements_for_executables, diff_entitlements_for_executables},
    executables::{
        __path_diff_executables_for_versions, __path_get_all_executables,
        __path_get_all_executables_entitlements, __path_get_executable_entitlements,
        __path_get_executable_versions, diff_executables_for_versions, get_all_executables,
        get_all_executables_entitlements, get_executable_entitlements, get_executable_versions,
    },
    frameworks::{
        __path_diff_frameworks_for_executables, __path_get_all_frameworks,
        __path_get_executable_frameworks, __path_get_framework_executables,
        __path_get_framework_versions,
    },
    operating_system_versions::{
        __path_get_extended_operating_system_versions,
        __path_get_operating_system_version_executables,
        __path_get_operating_system_version_frameworks, __path_get_operating_system_versions,
        __path_get_operating_system_versions_by_id, get_extended_operating_system_versions,
    },
    operating_systems::{__path_get_operating_system_by_id, __path_get_operating_systems},
    stats::__path_get_stats,
};
use axum::Router;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::models::AppState;

use super::{
    frameworks::{
        diff_frameworks_for_executables, get_all_frameworks, get_executable_frameworks,
        get_framework_executables, get_framework_versions,
    },
    operating_system_versions::{
        get_operating_system_version_executables, get_operating_system_version_frameworks,
        get_operating_system_versions, get_operating_system_versions_by_id,
    },
    operating_systems::{get_operating_system_by_id, get_operating_systems},
    stats::get_stats,
    tasks::{__path_get_running_tasks, get_running_tasks},
};

pub fn get_public_router(with_openapi: bool) -> Router<Arc<AppState>> {
    // Need to duplicate routes!() macro: https://github.com/juhaku/utoipa/issues/1372
    let (router, openapi): (Router<Arc<AppState>>, OpenApi) = OpenApiRouter::new()
        .routes(routes!(get_stats,))
        .routes(routes!(get_operating_systems))
        .routes(routes!(get_operating_system_by_id))
        .routes(routes!(get_devices))
        .routes(routes!(get_device_operating_system_versions))
        .routes(routes!(get_operating_system_versions))
        .routes(routes!(get_operating_system_versions_by_id))
        .routes(routes!(get_extended_operating_system_versions))
        .routes(routes!(get_operating_system_version_executables))
        .routes(routes!(get_operating_system_version_frameworks))
        .routes(routes!(get_executable_versions))
        .routes(routes!(get_all_executables))
        .routes(routes!(get_all_executables_entitlements))
        .routes(routes!(get_executable_entitlements))
        .routes(routes!(diff_executables_for_versions))
        .routes(routes!(diff_entitlements_for_executables))
        .routes(routes!(diff_frameworks_for_executables))
        .routes(routes!(get_executable_frameworks))
        .routes(routes!(get_all_frameworks))
        .routes(routes!(get_framework_versions))
        .routes(routes!(get_framework_executables))
        .routes(routes!(get_running_tasks))
        .split_for_parts();

    if with_openapi {
        log::info!("Serve public openapi documentation");

        router.merge(
            SwaggerUi::new("/swagger")
                .config(Config::from(format!("{PUBLIC_ROUTES_PREFIX}/openapi.json")))
                .url("/openapi.json", openapi),
        )
    } else {
        router
    }
}
