use std::sync::Arc;

use appledb_common::{
    api_models::ExtendedOperatingSystemVersions,
    db_models::{Framework, OperatingSystemVersion},
    routes::PublicRoutes,
};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemVersions,
    responses((status = OK, body = Vec<OperatingSystemVersion>))
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
    responses((status = OK, body = OperatingSystemVersion))
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

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemVersionsExtended,
    responses((status = OK, body = Vec<ExtendedOperatingSystemVersions>))
)]
pub async fn get_extended_operating_system_versions(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<ExtendedOperatingSystemVersions>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_extended_operating_system_versions()
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemVersionsExecutables,
    params(
        ("operating_system_version_id" = i32, description = "Operating system version identifier to get executables from"),
    ),
    responses((status = OK, body = Vec<crate::crud::ExecutableOperatingSystemVersion>))
)]
pub async fn get_operating_system_version_executables(
    State(state): State<Arc<AppState>>,
    Path(operating_system_version_id): Path<i64>,
) -> AppResult<Json<Vec<crate::crud::ExecutableOperatingSystemVersion>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_operating_system_version_executables(operating_system_version_id)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetOperatingSystemVersionsFrameworks,
    params(
        ("operating_system_version_id" = i32, description = "Operating system version identifier to get executables from"),
    ),
    responses((status = OK, body = Vec<Framework>))
)]
pub async fn get_operating_system_version_frameworks(
    State(state): State<Arc<AppState>>,
    Path(operating_system_version_id): Path<i64>,
) -> AppResult<Json<Vec<Framework>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_operating_system_version_frameworks(operating_system_version_id)
            .await?,
    ))
}
