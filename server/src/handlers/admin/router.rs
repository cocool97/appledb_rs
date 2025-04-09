use std::sync::Arc;

use appledb_common::routes::{
    POST_EXECUTABLE, POST_EXECUTABLE_ENTITLEMENTS_ROUTE, POST_OPERATING_SYSTEM_VERSION,
    STOP_RUNNING_TASK,
};
use axum::{
    Router,
    routing::{get, post},
};

use crate::models::AppState;

use super::{
    post_executable, post_executable_entitlements, post_operating_system_version,
    tasks::stop_running_task,
};

pub fn get_admin_router() -> Router<Arc<AppState>> {
    // TODO: apply JWT check layer here
    Router::new()
        .route(
            POST_EXECUTABLE_ENTITLEMENTS_ROUTE,
            post(post_executable_entitlements),
        )
        .route(POST_EXECUTABLE, post(post_executable))
        .route(
            POST_OPERATING_SYSTEM_VERSION,
            post(post_operating_system_version),
        )
        .route(STOP_RUNNING_TASK, get(stop_running_task))
}
