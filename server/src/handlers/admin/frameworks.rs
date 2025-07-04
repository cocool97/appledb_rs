use anyhow::anyhow;
use appledb_common::{
    IPSWFrameworks,
    api_models::{TaskProgress, TaskType},
    routes::AdminRoutes,
};
use axum::{Json, extract::State};
use sea_orm::SqlErr;
use serde::Serialize;
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    Result,
    crud::DBStatus,
    db_controller::DBController,
    models::AppState,
    utils::{AppError, AppResult},
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
    path = &AdminRoutes::PostExecutableFrameworks,
    responses((status = OK, body = String))
)]
pub async fn post_executable_frameworks(
    State(state): State<Arc<AppState>>,
    Json(frameworks): Json<IPSWFrameworks>,
) -> AppResult<Json<String>> {
    // Check if we can run this task
    {
        let running_tasks = state.running_tasks.read().await;
        if running_tasks.len() > state.max_concurrent_tasks {
            log::error!("Too many tasks running. Aborting this one");
            return AppResult::Err(AppError::from(anyhow!(
                "Too many tasks running. Aborting this one"
            )));
        }
    }

    let task_uuid = Uuid::new_v4();
    log::debug!("New task will spawn with uuid={task_uuid}...");

    let progress = Arc::new(RwLock::new(TaskProgress::new(
        TaskType::PostFrameworks,
        frameworks.executable_frameworks.len(),
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
    });

    // Add this task in database
    {
        let mut running_tasks = state.running_tasks.write().await;
        running_tasks.insert(task_uuid, (progress, task));
    }

    Ok(Json(task_uuid.to_string()))
}

async fn post_executable_frameworks_inner(
    db_controller: Arc<DBController>,
    progress: Arc<RwLock<TaskProgress>>,
    frameworks: IPSWFrameworks,
) -> Result<FrameworkInsertionStatus> {
    let operating_system_version = db_controller
        .crud_get_or_create_operating_system_version_by_platform_and_version(
            frameworks.platform.name().to_string(),
            frameworks.model_code,
            frameworks.version,
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
                                "Framework {} already exists for executable {}. Likely a twin...",
                                framework_id,
                                executable_status.db_identifier()
                            );
                            continue;
                        }
                        e => return Err(anyhow!("Unexpected database error: {:?}", e)),
                    }
                }
                return Err(anyhow!("Unexpected database error: {:?}", e));
            }
        }

        {
            let mut progress = progress.write().await;
            progress.done += 1;
        }

        log::debug!(
            "Added {} frameworks to executable {}",
            frameworks.len(),
            executable_status.db_identifier(),
        );
    }

    Ok(framework_insertion)
}
