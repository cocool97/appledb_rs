use std::{collections::HashSet, fmt::Display, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use appledb_common::{IPSWEntitlements, IPSWExecutableEntitlements, api_models::TaskProgress};
use axum::{Json, extract::State};
use sea_orm::SqlErr;
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    crud::DBStatus,
    db_controller::DBController,
    models::AppState,
    utils::{AppError, AppResult},
};

#[derive(Default, Debug)]
struct EntitlementsInsertionStatus {
    pub inserted_executables: u32,
    pub existing_executables: u32,
    pub inserted_entitlements: u32,
    pub existing_entitlements: u32,
}

impl Display for EntitlementsInsertionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "inserted_executables={},existing_executables={},inserted_entitlements={},existing_entitlements={}",
            self.inserted_executables,
            self.existing_executables,
            self.inserted_entitlements,
            self.existing_entitlements
        )
    }
}

fn format_entitlements(value: &Value) -> Result<HashSet<IPSWExecutableEntitlements>> {
    let mut entitlements = HashSet::new();

    match value {
        Value::Array(values) => {
            for value in values {
                entitlements = entitlements
                    .union(&format_entitlements(value)?)
                    .cloned()
                    .collect();
            }
        }
        Value::Object(dictionary) => {
            for (key, value) in dictionary {
                let sub_entitlements = format_entitlements(value)?;
                for ent in sub_entitlements {
                    if ent.key.is_empty() {
                        entitlements.insert(IPSWExecutableEntitlements {
                            key: key.clone(),
                            value: ent.value,
                        });
                    } else {
                        entitlements.insert(IPSWExecutableEntitlements {
                            key: format!("{}.{}", key, ent.key),
                            value: ent.value,
                        });
                    }
                }
            }
        }
        Value::Bool(b) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: b.to_string(),
            });
        }
        Value::Number(num) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: num.to_string(),
            });
        }
        Value::String(s) => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: s.clone(),
            });
        }
        Value::Null => {
            entitlements.insert(IPSWExecutableEntitlements {
                key: String::new(),
                value: "null".to_string(),
            });
        }
    }

    Ok(entitlements)
}

const TOKIO_TASK_SPAWN_DELAY: u64 = 5;

pub async fn post_executable_entitlements(
    State(state): State<Arc<AppState>>,
    Json(entitlements): Json<IPSWEntitlements>,
) -> AppResult<Json<String>> {
    // Check if we can run this task
    {
        let running_entitlements_tasks = state.running_entitlements_tasks.read().await;
        if running_entitlements_tasks.len() > state.max_concurrent_tasks {
            log::error!("Too many tasks running. Aborting this one");
            return AppResult::Err(AppError::from(anyhow!(
                "Too many tasks running. Aborting this one"
            )));
        }
    }

    let task_uuid = Uuid::new_v4();
    log::debug!("New task will spawn with uuid={task_uuid}...");

    let progress = Arc::new(RwLock::new(TaskProgress::new(
        entitlements.executable_entitlements.len(),
    )));

    let db_controller = state.db_controller.clone();
    let running_tasks = state.running_entitlements_tasks.clone();
    let task_progress = progress.clone();

    let task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(TOKIO_TASK_SPAWN_DELAY)).await;
        match post_executable_entitlements_inner(db_controller, task_progress, entitlements).await {
            Ok(res) => log::info!("Insertion OK: {}", res),
            Err(e) => log::error!("got error while inserting: {e}"),
        }

        // Remove this task from running tasks
        {
            let mut running_entitlements_tasks = running_tasks.write().await;
            running_entitlements_tasks.remove(&task_uuid);
        }
    });

    // Add this task in database
    {
        let mut running_entitlements_tasks = state.running_entitlements_tasks.write().await;
        running_entitlements_tasks.insert(task_uuid, (progress, task));
    }

    Ok(Json(task_uuid.to_string()))
}

async fn post_executable_entitlements_inner(
    db_controller: Arc<DBController>,
    progress: Arc<RwLock<TaskProgress>>,
    entitlements: IPSWEntitlements,
) -> Result<EntitlementsInsertionStatus> {
    let operating_system_version = db_controller
        .crud_get_or_create_operating_system_version_by_platform_and_version(
            entitlements.platform.name().to_string(),
            entitlements.model_code,
            entitlements.version,
        )
        .await?;

    let mut entitlements_insertion = EntitlementsInsertionStatus::default();

    for (executable, entitlements) in entitlements.executable_entitlements {
        let executable_status = db_controller
            .crud_get_or_create_executable(operating_system_version.id, &executable)
            .await?;

        match executable_status {
            DBStatus::AlreadyExists(executable_id) => {
                log::debug!("Executable {} already exists, skipping...", executable_id);
                entitlements_insertion.existing_executables += 1;
                {
                    let mut progress = progress.write().await;
                    progress.done += 1;
                }
                continue;
            }
            DBStatus::Created(_) => {
                entitlements_insertion.inserted_executables += 1;
            }
        }

        let entitlements = format_entitlements(&entitlements)?;
        for entitlement in &entitlements {
            let entitlement_id = match db_controller
                .crud_get_or_create_entitlement(&entitlement.key, &entitlement.value)
                .await?
            {
                DBStatus::AlreadyExists(id) => {
                    entitlements_insertion.existing_entitlements += 1;
                    id
                }
                DBStatus::Created(id) => {
                    entitlements_insertion.inserted_entitlements += 1;
                    id
                }
            };

            if let Err(e) = db_controller
                .crud_create_executable_entitlement(
                    executable_status.db_identifier(),
                    entitlement_id,
                )
                .await
            {
                if let Some(db_error) = e.sql_err() {
                    match db_error {
                        SqlErr::UniqueConstraintViolation(_) => {
                            log::warn!(
                                "Entitlement {} already exists for executable {}. Likely a twin...",
                                entitlement_id,
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
            "Added {} entitlements to executable {}",
            entitlements.len(),
            executable_status.db_identifier(),
        );
    }

    Ok(entitlements_insertion)
}
