use std::{collections::BTreeMap, sync::Arc};

use appledb_common::{
    api_models::ExecutableInfos,
    db_models::{Entitlement, Executable},
    routes::PublicRoutes,
};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{crud::ExecutableVersion, models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutableEntitlements,
    params(
        ("id" = i32, description = "Executable identifier"),
    ),
    responses((status = OK, body = Vec<Entitlement>))
)]
pub async fn get_executable_entitlements(
    State(state): State<Arc<AppState>>,
    Path(executable_id): Path<i32>,
) -> AppResult<Json<Vec<Entitlement>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_entitlements_for_executable(executable_id)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutableVersions,
    params(
        ("id" = i32, description = "Executable identifier"),
    ),
    responses((status = OK, body = Vec<ExecutableVersion>))
)]
pub async fn get_executable_versions(
    State(state): State<Arc<AppState>>,
    Path(executable_id): Path<i32>,
) -> AppResult<Json<Vec<ExecutableVersion>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_executable_versions(executable_id)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetAllExecutables,
    params(
    ),
    responses((status = OK, body = Vec<Executable>))
)]
pub async fn get_all_executables(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<Executable>>> {
    Ok(Json(state.db_controller.crud_get_all_executables().await?))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetAllExecutablesEntitlements,
    params(
        ("operating_system_version_id" = i32, description = "Operating system version identifier"),
    ),
    responses((status = OK, body = BTreeMap<String, Vec<Entitlement>>))
)]
pub async fn get_all_executables_entitlements(
    State(state): State<Arc<AppState>>,
    Path(operating_system_version_id): Path<i32>,
) -> AppResult<Json<BTreeMap<String, ExecutableInfos>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_all_executables_entitlements(operating_system_version_id)
            .await?,
    ))
}
