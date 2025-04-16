use std::{
    collections::{BTreeMap, HashSet},
    sync::Arc,
};

use appledb_common::{
    api_models::{Diff, ExecutableInfos},
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
        ("id" = i64, description = "Executable identifier"),
    ),
    responses((status = OK, body = Vec<Entitlement>))
)]
pub async fn get_executable_entitlements(
    State(state): State<Arc<AppState>>,
    Path(executable_id): Path<i64>,
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
        ("id" = i64, description = "Executable identifier"),
    ),
    responses((status = OK, body = Vec<ExecutableVersion>))
)]
pub async fn get_executable_versions(
    State(state): State<Arc<AppState>>,
    Path(executable_id): Path<i64>,
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
        ("operating_system_version_id" = i64, description = "Operating system version identifier"),
    ),
    responses((status = OK, body = BTreeMap<String, Vec<Entitlement>>))
)]
pub async fn get_all_executables_entitlements(
    State(state): State<Arc<AppState>>,
    Path(operating_system_version_id): Path<i64>,
) -> AppResult<Json<BTreeMap<String, ExecutableInfos>>> {
    Ok(Json(
        state
            .db_controller
            .crud_get_all_executables_entitlements(operating_system_version_id)
            .await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetDiffExecutablesOperatingSystemVersion,
    params(
        ("from_operating_system_version_id" = i64, description = "Initial operating_system_version identifier"),
        ("to_operating_system_version_id" = i64, description = "Final operating_system_version identifier"),
    ),
    responses((status = OK, body = Diff<Executable>))
)]
pub async fn diff_executables_for_versions(
    State(state): State<Arc<AppState>>,
    Path((from_operating_system_version_id, to_operating_system_version_id)): Path<(i64, i64)>,
) -> AppResult<Json<Diff<Executable>>> {
    let executables_from: HashSet<Executable> = state
        .db_controller
        .crud_get_operating_system_version_executables(from_operating_system_version_id)
        .await?
        .into_iter()
        .collect();

    let entitlements_to: HashSet<Executable> = state
        .db_controller
        .crud_get_operating_system_version_executables(to_operating_system_version_id)
        .await?
        .into_iter()
        .collect();

    let mut added = vec![];
    let mut removed = vec![];
    let mut unchanged = vec![];

    for executable in entitlements_to.iter() {
        if executables_from.contains(executable) {
            unchanged.push(executable.clone());
        } else {
            added.push(executable.clone());
        }
    }

    for entitlement in executables_from.iter() {
        if !entitlements_to.contains(entitlement) {
            removed.push(entitlement.clone());
        }
    }

    Ok(Json(Diff {
        added,
        removed,
        unchanged,
    }))
}
