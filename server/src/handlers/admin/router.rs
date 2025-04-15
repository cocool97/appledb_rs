use std::sync::Arc;

use appledb_common::routes::AdminRoutes;
use axum::{
    Router,
    routing::{post, put},
};
use strum::EnumCount;
use utoipa::OpenApi;

use utoipa_swagger_ui::{Config, SwaggerUi};

use super::{
    entitlements::post_executable_entitlements, frameworks::post_executable_frameworks,
    tasks::stop_running_task,
};
use crate::handlers::admin::entitlements::__path_post_executable_entitlements;
use crate::handlers::admin::frameworks::__path_post_executable_frameworks;
use crate::handlers::admin::tasks::__path_stop_running_task;
use crate::models::AppState;

pub fn setup_admin_openapi_router(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    log::info!("Serve admin openapi documentation");
    #[derive(OpenApi)]
    #[openapi(paths(
        post_executable_entitlements,
        stop_running_task,
        post_executable_frameworks
    ))]
    struct ApiDoc;

    // Update each path to add ADMIN_ROUTES prefix
    let mut openapi = ApiDoc::openapi();
    openapi.info.title = format!("{} - admin API documentation", env!("CARGO_PKG_NAME"));
    openapi.paths.paths = openapi
        .paths
        .paths
        .iter_mut()
        .map(|(path, item)| {
            (
                format!("{}{}", AdminRoutes::route_prefix(), path),
                item.to_owned(),
            )
        })
        .collect();

    // Check that every registered endpoint is documented (only in debug builds)
    debug_assert_eq!(
        openapi.paths.paths.len(),
        AdminRoutes::COUNT,
        "all admin handlers aren't documented..."
    );

    router.merge(
        SwaggerUi::new("/swagger")
            .config(Config::from(
                AdminRoutes::route_prefix().to_owned() + "/openapi.json",
            ))
            .url("/openapi.json", openapi),
    )
}

pub fn get_admin_router(with_openapi: bool) -> Router<Arc<AppState>> {
    let mut router = Router::new();

    if with_openapi {
        router = setup_admin_openapi_router(router);
    }

    router
        .route(
            &AdminRoutes::PostExecutableEntitlements.to_string(),
            post(post_executable_entitlements),
        )
        .route(
            &AdminRoutes::StopRunningTask.to_string(),
            put(stop_running_task),
        )
        .route(
            &AdminRoutes::PostExecutableFrameworks.to_string(),
            post(post_executable_frameworks),
        )
}
