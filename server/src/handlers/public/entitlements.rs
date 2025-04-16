use std::{collections::HashSet, sync::Arc, vec};

use appledb_common::{api_models::Diff, db_models::Entitlement, routes::PublicRoutes};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetDiffEntitlementsExecutables,
    params(
        ("from_executable_id" = i64, description = "Initial executable identifier"),
        ("to_executable_id" = i64, description = "Final executable identifier"),
    ),
    responses((status = OK, body = Diff<Entitlement>))
)]
pub async fn diff_entitlements_for_executables(
    State(state): State<Arc<AppState>>,
    Path((from_executable_id, to_executable_id)): Path<(i64, i64)>,
) -> AppResult<Json<Diff<Entitlement>>> {
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

    Ok(Json(Diff {
        added,
        removed,
        unchanged,
    }))
}
