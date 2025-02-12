use std::sync::Arc;

use appledb_common::{api_models::AppResponse, db_models::OperatingSystem, routes::PublicRoutes};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystems,
    responses((status = OK, body = AppResponse<Vec<OperatingSystem>>))
)]
pub async fn get_operating_systems(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<AppResponse<Vec<OperatingSystem>>>> {
    Ok(Json(AppResponse {
        data: state.db_controller.crud_get_operating_systems().await?,
    }))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemById,
    params(
        ("id" = i32, description = "Operating system identifier to retrieve"),
    ),
    responses((status = OK, body = AppResponse<OperatingSystem>))
)]
pub async fn get_operating_system_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<AppResponse<OperatingSystem>>> {
    Ok(Json(AppResponse {
        data: state
            .db_controller
            .crud_get_operating_system_by_id(id)
            .await?,
    }))
}
