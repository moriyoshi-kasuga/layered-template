use api_shared::error::ApiError;
use axum::response::IntoResponse;
use serde::Serialize;

use super::{failure_response::IntoFailureResponse, success_response::SuccessResponse};

pub type ApiResponse<T> = ApiErrorResponse<T, ApiError>;

#[derive(Debug, Clone)]
pub struct ApiErrorResponse<T, E>(&'static str, Result<T, E>);

impl<T, E> ApiErrorResponse<T, E> {
    pub fn new(message: &'static str, result: Result<T, E>) -> Self {
        Self(message, result)
    }
}

impl<T, E> From<(&'static str, Result<T, E>)> for ApiErrorResponse<T, E> {
    fn from(value: (&'static str, Result<T, E>)) -> Self {
        Self(value.0, value.1)
    }
}

impl<T, E> From<ApiErrorResponse<T, E>> for (&'static str, Result<T, E>) {
    fn from(value: ApiErrorResponse<T, E>) -> Self {
        (value.0, value.1)
    }
}

impl<T: Serialize, E: Serialize + IntoFailureResponse> IntoResponse for ApiErrorResponse<T, E> {
    fn into_response(self) -> axum::response::Response {
        match self.1 {
            Ok(ok) => SuccessResponse::new(self.0, ok).into_response(),
            Err(err) => err.into_failure_response().into_response(),
        }
    }
}
