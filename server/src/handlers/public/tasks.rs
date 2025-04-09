use std::{collections::HashMap, sync::Arc};

use appledb_common::{api_models::TaskProgress, routes::PublicRoutes};
use axum::{Json, extract::State};

use crate::{models::AppState, utils::AppResult};

#[utoipa::path(
    get,
    path = PublicRoutes::GetRunningTasks,
    responses((status = OK, body = BTreeMap<String, TaskProgress>))
)]
pub async fn get_running_tasks(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<HashMap<String, TaskProgress>>> {
    let running_tasks = {
        let tasks = state.running_entitlements_tasks.read().await;
        let mut running_tasks = HashMap::new();
        for (task_uuid, (progress, _)) in tasks.iter() {
            let task = progress.read().await;

            running_tasks.insert(task_uuid.to_string(), task.clone());
        }

        running_tasks
    };

    Ok(Json(running_tasks))
}
