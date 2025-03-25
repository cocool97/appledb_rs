use std::sync::Arc;

use appledb_common::operating_system_version::CreateOperatingSystemVersion;
use axum::{Json, extract::State};

use crate::{models::AppState, utils::AppResult};

pub async fn post_operating_system_version(
    State(state): State<Arc<AppState>>,
    Json(os_version_request): Json<CreateOperatingSystemVersion>,
) -> AppResult<Json<i32>> {
    let os_version_id = state
        .db_controller
        .crud_create_operating_system_version(
            os_version_request.operating_system_id,
            os_version_request.device_id,
            os_version_request.version,
        )
        .await?;

    Ok(Json(os_version_id))
}
