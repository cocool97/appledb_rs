use std::sync::Arc;

use appledb_common::{api_models::AppResponse, db_models::Executable, routes::PublicRoutes};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutables,
    responses((status = OK, body = AppResponse<Vec<Executable>>))
)]
pub async fn get_executables(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<Executable>>> {
    Ok(Json(state.db_controller.crud_get_executables().await?))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutablesById,
    params(
        ("id" = i32, description = "Executable identifier to retrieve"),
    ),
    responses((status = OK, body = AppResponse<Executable>))
)]
pub async fn get_executables_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Executable>> {
    Ok(Json(
        state.db_controller.crud_get_executable_by_id(id).await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutablesByName,
    params(
        ("name" = i32, description = "Executables name to retrieve"),
    ),
    responses((status = OK, body = AppResponse<Vec<Executable>>))
)]
pub async fn get_executables_by_name(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> AppResult<Json<Vec<Executable>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_executables_by_name(name)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutablesWithEntitlement,
    params(
        ("operating_system_version_id" = i32, description = "Operating system version identifier to retrieve"),
        ("entitlement_key" = String, description = "Entitlement key value to retrieve")
    ),
    responses((status = OK, body = AppResponse<Executable>))
)]
pub async fn get_executables_with_entitlement_for_os_version(
    State(state): State<Arc<AppState>>,
    Path((operating_system_version_id, entitlement_key)): Path<(i32, String)>,
) -> AppResult<Json<Vec<Executable>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_executables_with_entitlement_for_os_version(
                operating_system_version_id,
                entitlement_key,
            )
            .await?,
    ))
}
