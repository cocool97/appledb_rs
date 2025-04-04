use std::{collections::HashSet, sync::Arc};

use anyhow::{Result, anyhow};
use appledb_common::{
    IPSWEntitlements, IPSWExecutableEntitlements, api_models::EntitlementsInsertionStatus,
};
use axum::{Json, extract::State};
use sea_orm::SqlErr;
use serde_json::Value;

use crate::{crud::DBStatus, models::AppState, utils::AppResult};

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

// #[axum_macros::debug_handler]
pub async fn post_executable_entitlements(
    State(state): State<Arc<AppState>>,
    Json(entitlements): Json<IPSWEntitlements>,
) -> AppResult<Json<EntitlementsInsertionStatus>> {
    let operating_system_version = state
        .db_controller
        .crud_get_or_create_operating_system_version_by_platform_and_version(
            entitlements.platform.name().to_string(),
            entitlements.model_code,
            entitlements.version,
        )
        .await?;

    let mut entitlements_insertion = EntitlementsInsertionStatus::default();

    for (executable, entitlements) in entitlements.executable_entitlements {
        let executable_status = state
            .db_controller
            .crud_get_or_create_executable(operating_system_version.id, &executable)
            .await?;

        match executable_status {
            DBStatus::AlreadyExists(executable_id) => {
                log::warn!("Executable {} already exists, skipping...", executable_id);
                entitlements_insertion.existing_executables += 1;
                continue;
            }
            DBStatus::Created(_) => {
                entitlements_insertion.inserted_executables += 1;
            }
        }

        let entitlements = format_entitlements(&entitlements)?;
        for entitlement in &entitlements {
            let entitlement_id = match state
                .db_controller
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

            if let Err(e) = state
                .db_controller
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
                        e => return Err(anyhow!("Unexpected database error: {:?}", e).into()),
                    }
                }
                return Err(anyhow!("Unexpected database error: {:?}", e).into());
            }
        }

        log::info!(
            "Added {} entitlements to executable {}",
            entitlements.len(),
            executable_status.db_identifier(),
        );
    }

    Ok(Json(entitlements_insertion))
}
