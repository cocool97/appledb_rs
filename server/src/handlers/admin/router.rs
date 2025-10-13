use std::sync::Arc;

use appledb_common::routes::ADMIN_ROUTES_PREFIX;
use axum::Router;

use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::{Config, SwaggerUi};

use super::{
    entitlements::post_executable_entitlements, frameworks::post_executable_frameworks,
    tasks::stop_running_task,
};
use crate::handlers::admin::entitlements::__path_post_executable_entitlements;
use crate::handlers::admin::frameworks::__path_post_executable_frameworks;
use crate::handlers::admin::tasks::__path_stop_running_task;
use crate::models::AppState;

pub fn get_admin_router(with_openapi: bool) -> Router<Arc<AppState>> {
    // Need to duplicate routes!() macro: https://github.com/juhaku/utoipa/issues/1372
    let (router, openapi): (Router<Arc<AppState>>, OpenApi) = OpenApiRouter::new()
        .routes(routes!(post_executable_entitlements,))
        .routes(routes!(stop_running_task))
        .routes(routes!(post_executable_frameworks))
        .split_for_parts();

    if with_openapi {
        log::info!("Serve admin openapi documentation");

        router.merge(
            SwaggerUi::new("/swagger")
                .config(Config::from(format!("{ADMIN_ROUTES_PREFIX}/openapi.json")))
                .url("/openapi.json", openapi),
        )
    } else {
        router
    }
}
