use crate::helper::serialize_status_code;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SuccessResponse<T> {
    #[serde(serialize_with = "serialize_status_code")]
    status: StatusCode,
    message: &'static str,
    data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(message: &'static str, data: T) -> Self {
        Self {
            status: StatusCode::OK,
            message,
            data,
        }
    }

    pub fn with_status(message: &'static str, data: T, status: StatusCode) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}
