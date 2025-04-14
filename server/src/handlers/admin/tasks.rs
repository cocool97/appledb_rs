use std::{str::FromStr, sync::Arc};

use anyhow::anyhow;
use appledb_common::routes::AdminRoutes;
use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    models::AppState,
    utils::{AppError, AppResult},
};

#[utoipa::path(
    put,
    path = &AdminRoutes::StopRunningTask,
    responses((status = OK, body = ()))
)]
pub async fn stop_running_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> AppResult<()> {
    let uuid =
        Uuid::from_str(&task_id).map_err(|_| anyhow!("provided task_id not not an uuidv4"))?;
    let task = {
        let mut tasks = state.running_tasks.write().await;
        tasks.remove(&uuid)
    };

    match task {
        Some((_, handle)) => {
            handle.abort();
            log::info!("successfully aborted task {uuid}");
            Ok(())
        }
        None => Err(AppError::from(anyhow!("unknown task with id {uuid}"))),
    }
}
