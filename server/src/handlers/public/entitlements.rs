use std::{collections::HashSet, sync::Arc, vec};

use appledb_common::{
    api_models::{AppResponse, EntitlementsDiff},
    db_models::Entitlement,
    routes::PublicRoutes,
};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetEntitlements,
    responses((status = OK, body = AppResponse<Vec<Entitlement>>))
)]
pub async fn get_entitlements(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<Entitlement>>> {
    Ok(Json(state.db_controller.crud_get_entitlements().await?))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetEntitlementsById,
    params(
        ("id" = i32, description = "Entitlement identifier"),
    ),
    responses((status = OK, body = AppResponse<Entitlement>))
)]
pub async fn get_entitlements_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Entitlement>> {
    Ok(Json(
        state.db_controller.crud_get_entitlement_by_id(id).await?,
    ))
}

#[utoipa::path(
    get,
    path = PublicRoutes::GetEntitlementsForExecutable,
    params(
        ("id" = i32, description = "Executable identifier"),
    ),
    responses((status = OK, body = AppResponse<Vec<Entitlement>>))
)]
pub async fn get_entitlements_for_executable(
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
    path = PublicRoutes::DiffEntitlementsExecutables,
    params(
        ("from_executable_id" = i32, description = "Initial executable identifier"),
        ("to_executable_id" = i32, description = "Final executable identifier"),
    ),
    responses((status = OK, body = AppResponse<EntitlementsDiff>))
)]
pub async fn diff_entitlements_for_executables(
    State(state): State<Arc<AppState>>,
    Path((from_executable_id, to_executable_id)): Path<(i32, i32)>,
) -> AppResult<Json<EntitlementsDiff>> {
    let entitlements_from: HashSet<Entitlement> = state
        .db_controller
        .crud_get_entitlements_for_executable(from_executable_id)
        .await?
        .into_iter()
        .collect();

    let entitlements_to: HashSet<Entitlement> = state
        .db_controller
        .crud_get_entitlements_for_executable(to_executable_id)
        .await?
        .into_iter()
        .collect();

    let mut added = vec![];
    let mut removed = vec![];
    let mut unchanged = vec![];

    for entitlement in entitlements_to.iter() {
        if entitlements_from.contains(entitlement) {
            unchanged.push(entitlement.clone());
        } else {
            added.push(entitlement.clone());
        }
    }

    for entitlement in entitlements_from.iter() {
        if !entitlements_to.contains(entitlement) {
            removed.push(entitlement.clone());
        }
    }

    Ok(Json(EntitlementsDiff {
        added,
        removed,
        unchanged,
    }))
}
