use anyhow::anyhow;
use appledb_common::{
    IPSWFrameworks,
    api_models::{TaskProgress, TaskSource, TaskType},
};
use axum::{Json, extract::State};
use sea_orm::SqlErr;
use serde::Serialize;
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    Result, crud::DBStatus, db_controller::DBController, models::AppState, utils::AppResult,
};

const TOKIO_TASK_SPAWN_DELAY: u64 = 5;

#[derive(Default, Serialize)]
pub struct FrameworkInsertionStatus {
    pub existing_frameworks: u32,
    pub inserted_frameworks: u32,
}

impl Display for FrameworkInsertionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "existing_frameworks={},inserted_frameworks={}",
            self.existing_frameworks, self.inserted_frameworks
        )
    }
}

#[utoipa::path(
    post,
    path = "/executable/frameworks",
    responses((status = OK, body = String))
)]
pub async fn post_executable_frameworks(
    State(state): State<Arc<AppState>>,
    Json(frameworks): Json<IPSWFrameworks>,
) -> AppResult<Json<String>> {
    let task_uuid = post_executable_frameworks_public(state, frameworks, TaskSource::Api).await?;
    Ok(Json(task_uuid.to_string()))
}

pub async fn post_executable_frameworks_public(
    state: Arc<AppState>,
    frameworks: IPSWFrameworks,
    task_source: TaskSource,
) -> Result<Uuid> {
    let task_uuid = Uuid::new_v4();
    log::debug!("New task will spawn with uuid={task_uuid}...");

    let progress = Arc::new(RwLock::new(TaskProgress::new(
        TaskType::PostFrameworks,
        task_source.to_string(),
        frameworks.executable_frameworks.len() as u64,
    )));

    let db_controller = state.db_controller.clone();
    let running_tasks = state.running_tasks.clone();
    let task_progress = progress.clone();

    let task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(TOKIO_TASK_SPAWN_DELAY)).await;
        match post_executable_frameworks_inner(db_controller, task_progress, frameworks).await {
            Ok(res) => log::info!("Insertion OK: {res}",),
            Err(e) => log::error!("got error while inserting: {e}"),
        }

        // Remove this task from running tasks
        {
            let mut running_tasks = running_tasks.write().await;
            running_tasks.remove(&task_uuid);
        }

        // Drop semaphore to let other tasks do the job if this is a LocalTask
        if let TaskSource::Local(permit) = task_source {
            drop(permit);
            log::debug!("released semaphore permit...");
        }
    });

    // Add this task in database
    {
        let mut running_tasks = state.running_tasks.write().await;
        running_tasks.insert(task_uuid, (progress, task));
    }

    Ok(task_uuid)
}

async fn post_executable_frameworks_inner(
    db_controller: Arc<DBController>,
    progress: Arc<RwLock<TaskProgress>>,
    frameworks: IPSWFrameworks,
) -> Result<FrameworkInsertionStatus> {
    let operating_system_version = db_controller
        .crud_get_or_create_operating_system_version_by_platform_and_version(
            &frameworks.platform.to_string(),
            &frameworks.model_code,
            &frameworks.version,
        )
        .await?;

    let mut framework_insertion = FrameworkInsertionStatus::default();

    for (executable, frameworks) in frameworks.executable_frameworks {
        let executable_status = db_controller
            .crud_get_or_create_executable(operating_system_version.id, &executable)
            .await?;

        for framework_full_path in &frameworks {
            let framework_id = match db_controller
                .crud_get_or_create_framework(framework_full_path)
                .await?
            {
                DBStatus::AlreadyExists(id) => {
                    framework_insertion.existing_frameworks += 1;
                    id
                }
                DBStatus::Created(id) => {
                    framework_insertion.inserted_frameworks += 1;
                    id
                }
            };

            if let Err(e) = db_controller
                .crud_create_executable_framework(executable_status.db_identifier(), framework_id)
                .await
            {
                if let Some(db_error) = e.sql_err() {
                    match db_error {
                        SqlErr::UniqueConstraintViolation(_) => {
                            log::debug!(
                                "Framework {framework_id} already exists for executable {}. Likely a twin...",
                                executable_status.db_identifier()
                            );
                            continue;
                        }
                        e => return Err(anyhow!("Unexpected database error: {e:?}")),
                    }
                }
                return Err(anyhow!("Unexpected database error: {e:?}"));
            }
        }

        {
            let mut progress = progress.write().await;
            progress.increment_done();
        }

        log::debug!(
            "Added {} frameworks to executable {}",
            frameworks.len(),
            executable_status.db_identifier(),
        );
    }

    Ok(framework_insertion)
}
