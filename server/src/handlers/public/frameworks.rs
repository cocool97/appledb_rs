use std::{collections::HashSet, sync::Arc};

use appledb_common::{api_models::Diff, db_models::Framework, routes::PublicRoutes};
use axum::{
    Json,
    extract::{Path, State},
};

use crate::{models::AppState, utils::AppResult};

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
