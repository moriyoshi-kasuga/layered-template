use crate::helper::serialize_status_code;
use api_shared::error::ApiError;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct FailureResponse<E> {
    #[serde(serialize_with = "serialize_status_code")]
    status: StatusCode,
    message: &'static str,
    error: E,
}

impl<E> FailureResponse<E> {
    pub fn new(status: StatusCode, message: &'static str, error: E) -> Self {
        Self {
            status,
            message,
            error,
        }
    }
}

impl<E: Serialize> IntoResponse for FailureResponse<E> {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}

pub trait IntoFailureResponse {
    type E: Serialize;
    fn into_failure_response(self) -> FailureResponse<Self::E>;
}

impl IntoFailureResponse for ApiError {
    type E = String;

    fn into_failure_response(self) -> FailureResponse<Self::E> {
        match &self {
            ApiError::InvalidRequest(_) => {
                FailureResponse::new(StatusCode::BAD_REQUEST, "Client Error", self.to_string())
            }
            ApiError::ServerError(_) => FailureResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
                self.to_string(),
            ),
            ApiError::DbError(_) => FailureResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
                self.to_string(),
            ),
        }
    }
}
