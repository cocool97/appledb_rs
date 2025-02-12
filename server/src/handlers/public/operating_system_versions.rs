use std::sync::Arc;

use appledb_common::{
    api_models::AppResponse, db_models::OperatingSystemVersion, routes::PublicRoutes,
};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemVersions,
    responses((status = OK, body = AppResponse<Vec<OperatingSystemVersion>>))
)]
pub async fn get_operating_system_versions(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<OperatingSystemVersion>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_operating_system_version()
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemVersionsById,
    params(
        ("id" = i32, description = "Operating system version identifier to retrieve"),
    ),
    responses((status = OK, body = AppResponse<OperatingSystemVersion>))
)]
pub async fn get_operating_system_versions_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<OperatingSystemVersion>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_operating_system_version_by_id(id)
            .await?,
    ))
}
