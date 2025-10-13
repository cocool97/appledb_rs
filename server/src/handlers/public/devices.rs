use std::sync::Arc;

use appledb_common::db_models::{Device, OperatingSystemVersion};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = "/devices/all",
    responses((status = OK, body = Vec<Device>))
)]
pub async fn get_devices(State(state): State<Arc<AppState>>) -> AppResult<Json<Vec<Device>>> {
    Ok(Json(state.db_controller.crud_get_devices().await?))
}

#[utoipa::path(
    get,
    path = "/device/{id}/operating_system_versions",
    responses((status = OK, body = Vec<OperatingSystemVersion>))
)]
pub async fn get_device_operating_system_versions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Vec<OperatingSystemVersion>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_device_operating_system_versions(id)
            .await?,
    ))
}
