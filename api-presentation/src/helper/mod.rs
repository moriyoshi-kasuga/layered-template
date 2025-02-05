use axum::http::StatusCode;
use serde::Serializer;

pub mod api_response;
pub mod failure_response;
pub mod success_response;

fn serialize_status_code<S: Serializer>(
    status_code: &StatusCode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(status_code.as_u16())
}
