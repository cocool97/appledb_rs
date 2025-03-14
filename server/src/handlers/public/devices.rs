use std::sync::Arc;

use appledb_common::{api_models::AppResponse, db_models::Device, routes::PublicRoutes};
use axum::{Json, extract::State};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetDevices,
    responses((status = OK, body = AppResponse<Vec<Device>>))
)]
pub async fn get_devices(State(state): State<Arc<AppState>>) -> AppResult<Json<Vec<Device>>> {
    Ok(Json(state.db_controller.crud_get_devices().await?))
}
