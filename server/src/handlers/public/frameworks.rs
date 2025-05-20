use std::{collections::HashSet, sync::Arc};

use appledb_common::{
    api_models::Diff,
    db_models::{Executable, Framework},
    routes::PublicRoutes,
};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{crud::OperatingSystemVersionExtended, models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetExecutableFrameworks,
    params(
        ("executable_operating_system_id" = i64, description = "Executable identifier"),
    ),
    responses((status = OK, body = Vec<Framework>))
)]
pub async fn get_executable_frameworks(
    State(state): State<Arc<AppState>>,
    Path(executable_operating_system_id): Path<i64>,
) -> AppResult<Json<Vec<Framework>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_frameworks_for_executable(executable_operating_system_id)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetDiffFrameworksExecutables,
    params(
        ("from_executable_id" = i64, description = "Initial executable identifier"),
        ("to_executable_id" = i64, description = "Final executable identifier"),
    ),
    responses((status = OK, body = Diff<Framework>))
)]
pub async fn diff_frameworks_for_executables(
    State(state): State<Arc<AppState>>,
    Path((from_executable_id, to_executable_id)): Path<(i64, i64)>,
) -> AppResult<Json<Diff<Framework>>> {
    let frameworks_from: HashSet<Framework> = state
        .db_controller
        .crud_get_frameworks_for_executable(from_executable_id)
        .await?
        .into_iter()
        .collect();

    let frameworks_to: HashSet<Framework> = state
        .db_controller
        .crud_get_frameworks_for_executable(to_executable_id)
        .await?
        .into_iter()
        .collect();

    let mut added = vec![];
    let mut removed = vec![];
    let mut unchanged = vec![];

    for framework in frameworks_to.iter() {
        if frameworks_from.contains(framework) {
            unchanged.push(framework.clone());
        } else {
            added.push(framework.clone());
        }
    }

    for framework in frameworks_from.iter() {
        if !frameworks_to.contains(framework) {
            removed.push(framework.clone());
        }
    }

    Ok(Json(Diff {
        added,
        removed,
        unchanged,
    }))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetAllFrameworks,
    params(
    ),
    responses((status = OK, body = Vec<Framework>))
)]
pub async fn get_all_frameworks(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<Framework>>> {
    Ok(Json(state.db_controller.crud_get_all_frameworks().await?))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetFrameworkVersions,
    params(
        ("id" = i64, description = "Framework identifier"),
    ),
    responses((status = OK, body = Vec<OperatingSystemVersionExtended>))
)]
pub async fn get_framework_versions(
    State(state): State<Arc<AppState>>,
    Path(framework_id): Path<i64>,
) -> AppResult<Json<Vec<OperatingSystemVersionExtended>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_framework_versions(framework_id)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetFrameworkExecutables,
    params(
        ("framework_id" = i64, description = "Framework identifier"),
        ("operating_system_version_id" = i64, description = "Operating system version identifier"),
    ),
    responses((status = OK, body = Vec<Executable>))
)]
pub async fn get_framework_executables(
    State(state): State<Arc<AppState>>,
    Path((framework_id, operating_system_version_id)): Path<(i64, i64)>,
) -> AppResult<Json<Vec<Executable>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_framework_executables(framework_id, operating_system_version_id)
            .await?,
    ))
}
