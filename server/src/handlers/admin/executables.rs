use std::sync::Arc;

use appledb_common::{api_models::AppResponse, executable::CreateExecutable};
use axum::{Json, extract::State};

use crate::{crud::DBStatus, models::AppState, utils::AppResult};

// #[axum_macros::debug_handler]
pub async fn post_executable(
    State(state): State<Arc<AppState>>,
    Json(executable_request): Json<CreateExecutable>,
) -> AppResult<Json<AppResponse<DBStatus>>> {
    let executable_id = state
        .db_controller
        .crud_get_or_create_executable(
            executable_request.operating_system_version_id,
            executable_request.name,
        )
        .await?;

    Ok(Json(AppResponse {
        data: executable_id,
    }))
}
