use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Mongo(mongodb::error::Error),
    Elapsed(tower::timeout::error::Elapsed),
    Tokio(TokioError),
}

#[derive(Debug)]
pub enum TokioError {
    Elapsed(tokio::time::error::Elapsed),
    JoinError(tokio::task::JoinError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Mongo(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            AppError::Elapsed(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            AppError::Tokio(TokioError::Elapsed(error)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, error.to_string())
            }
            AppError::Tokio(TokioError::JoinError(error)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, error.to_string())
            }
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}

impl From<tower::timeout::error::Elapsed> for AppError {
    fn from(inner: tower::timeout::error::Elapsed) -> Self {
        AppError::Elapsed(inner)
    }
}

impl From<tokio::time::error::Elapsed> for AppError {
    fn from(inner: tokio::time::error::Elapsed) -> Self {
        AppError::Tokio(TokioError::Elapsed(inner))
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(inner: tokio::task::JoinError) -> Self {
        AppError::Tokio(TokioError::JoinError(inner))
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(inner: mongodb::error::Error) -> Self {
        AppError::Mongo(inner)
    }
}
