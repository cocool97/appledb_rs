use std::sync::Arc;

use appledb_common::{db_models::OperatingSystem, routes::PublicRoutes};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystems,
    responses((status = OK, body = Vec<OperatingSystem>))
)]
pub async fn get_operating_systems(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<OperatingSystem>>> {
    Ok(Json(
        state.db_controller.crud_get_operating_systems().await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemById,
    params(
        ("id" = i32, description = "Operating system identifier to retrieve"),
    ),
    responses((status = OK, body = OperatingSystem))
)]
pub async fn get_operating_system_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<OperatingSystem>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_operating_system_by_id(id)
            .await?,
    ))
}
