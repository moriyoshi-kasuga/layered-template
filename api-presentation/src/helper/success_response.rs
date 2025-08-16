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
    pub const fn new(status: StatusCode, message: &'static str, data: T) -> Self {
        Self {
            status,
            message,
            data,
        }
    }

    pub const fn ok(message: &'static str, data: T) -> Self {
        Self {
            status: StatusCode::OK,
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

pub trait IntoSuccessResponse {
    type T: Serialize;
    fn into_success_response(self) -> SuccessResponse<Self::T>;
}

impl<T: Serialize> IntoSuccessResponse for (&'static str, T) {
    type T = T;

    fn into_success_response(self) -> SuccessResponse<T> {
        SuccessResponse::ok(self.0, self.1)
    }
}

impl<T: Serialize> IntoSuccessResponse for (StatusCode, &'static str, T) {
    type T = T;

    fn into_success_response(self) -> SuccessResponse<Self::T> {
        SuccessResponse::new(self.0, self.1, self.2)
    }
}
