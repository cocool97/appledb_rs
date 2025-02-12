use anyhow::Result;
use appledb_common::api_models::ServerErrorResponse;
use axum::{http::StatusCode, response::IntoResponse};

pub type AppResult<T> = Result<T, AppError>;

pub struct AppError {
    e: anyhow::Error,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = ServerErrorResponse {
            reason: self.e.to_string(),
        };
        let mut response = axum::Json(body).into_response();
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

        response
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        Self { e }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        Self {
            e: anyhow::anyhow!(e),
        }
    }
}
