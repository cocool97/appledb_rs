use std::sync::Arc;

use appledb_common::{routes::PublicRoutes, server_stats::ServerStats};
use axum::{Json, extract::State};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetStats,
    responses((status = OK, body = ServerStats))
)]
pub async fn get_stats(State(state): State<Arc<AppState>>) -> AppResult<Json<ServerStats>> {
    Ok(Json(ServerStats {
        known_devices: state.db_controller.crud_get_devices_count().await?,
        known_operating_system_versions: state
            .db_controller
            .crud_get_operating_system_version_count()
            .await?,
        known_entitlements: state.db_controller.crud_get_entitlements_count().await?,
        known_executables: state.db_controller.crud_get_executables_count().await?,
        known_frameworks: state.db_controller.crud_get_frameworks_count().await?,
    }))
}
