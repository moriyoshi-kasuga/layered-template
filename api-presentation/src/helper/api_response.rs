use api_shared::error::ApiError;
use axum::response::IntoResponse;
use serde::Serialize;

use super::{failure_response::IntoFailureResponse, success_response::SuccessResponse};

pub type ApiResponse<T> = ApiErrorResponse<T, ApiError>;

#[derive(Debug, Clone)]
pub struct ApiErrorResponse<T, E>(Result<(&'static str, T), E>);

impl<T, E> ApiErrorResponse<T, E> {
    /// multiple message response
    ///
    /// ```rust
    /// # use api_presentation::helper::api_response::ApiResponse;
    /// struct ItemResponse {
    ///     item_name: String,
    /// }
    ///
    /// // some graceful process
    /// let result = {
    ///     Ok(ItemResponse { item_name: String::from("Item Name") })
    /// };
    ///
    /// let result = result.map(|v| {
    ///     let message = match v.item_name.as_str() {
    ///         "Item Name" => "Successfully get Item Name!",
    ///         _ => "Successfully get item",
    ///     };
    ///     (message, v)
    /// });
    ///
    /// ApiResponse::new(result);
    /// ```
    ///
    /// * `result`: processed data
    pub fn new(result: Result<(&'static str, T), E>) -> Self {
        Self(result)
    }

    /// const message response
    ///
    /// ```rust
    /// # use api_presentation::helper::api_response::ApiResponse;
    /// struct ItemResponse {
    ///     item_name: String,
    /// }
    ///
    /// // some graceful process
    /// let result = {
    ///     Ok(ItemResponse { item_name: String::from("Item Name") })
    /// };
    ///
    /// ApiResponse::message("Successfully get item", result);
    ///
    /// ```
    /// * `message`: success message
    /// * `result`: processed data
    pub fn message(message: &'static str, result: Result<T, E>) -> Self {
        Self(result.map(|v| (message, v)))
    }
}

impl<T, E> From<(&'static str, Result<T, E>)> for ApiErrorResponse<T, E> {
    fn from(value: (&'static str, Result<T, E>)) -> Self {
        Self::message(value.0, value.1)
    }
}

impl<T: Serialize, E: Serialize + IntoFailureResponse> IntoResponse for ApiErrorResponse<T, E> {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            Ok(ok) => SuccessResponse::new(ok.0, ok.1).into_response(),
            Err(err) => err.into_failure_response().into_response(),
        }
    }
}
