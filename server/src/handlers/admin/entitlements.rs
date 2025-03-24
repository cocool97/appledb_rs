use std::sync::Arc;

use anyhow::anyhow;
use appledb_common::{IPSWEntitlements, api_models::AppResponse};
use axum::{Json, extract::State};
use sea_orm::SqlErr;

use crate::{crud::DBStatus, models::AppState, utils::AppResult};

// #[axum_macros::debug_handler]
pub async fn post_executable_entitlements(
    State(state): State<Arc<AppState>>,
    Json(entitlements): Json<IPSWEntitlements>,
) -> AppResult<Json<AppResponse<String>>> {
    let operating_system_version = state
        .db_controller
        .crud_get_or_create_operating_system_version_by_platform_and_version(
            entitlements.platform.name().to_string(),
            entitlements.model_code,
            entitlements.version,
        )
        .await?;

    for (executable, entitlements) in entitlements.executable_entitlements {
        let executable_status = state
            .db_controller
            .crud_get_or_create_executable(operating_system_version.id, &executable)
            .await?;
        log::info!("Created executable {executable}");

        if let DBStatus::AlreadyExists(executable_id) = executable_status {
            log::warn!("Executable {} already exists, skipping...", executable_id);
            continue;
        }

        let mut created = 0;
        let mut existing = 0;
        for entitlement in &entitlements {
            let entitlement_id = match state
                .db_controller
                .crud_get_or_create_entitlement(&entitlement.key, &entitlement.value)
                .await?
            {
                DBStatus::AlreadyExists(id) => {
                    existing += 1;
                    id
                }
                DBStatus::Created(id) => {
                    created += 1;
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
            "Added {} entitlements to executable {} - created: {} existing: {}",
            entitlements.len(),
            executable_status.db_identifier(),
            created,
            existing
        );
    }

    Ok(Json(AppResponse {
        data: "ok".to_string(),
    }))
}
