use api_shared::error::ApiError;
use axum::response::IntoResponse;

use super::{failure_response::IntoFailureResponse, success_response::IntoSuccessResponse};

pub type ApiResponse<T> = ApiMessageResponse<T, ApiError>;
pub type ApiMessageResponse<T, E> = ApiBaseResponse<(&'static str, T), E>;

#[derive(Debug, Clone)]
pub struct ApiBaseResponse<T, E>(Result<T, E>);

impl<T, E> ApiBaseResponse<T, E> {
    /// multiple message response
    ///
    /// ```rust
    /// # use api_presentation::helper::api_response::ApiBaseResponse;
    /// struct ItemResponse {
    ///     item_name: String,
    /// }
    ///
    /// let result: Result<ItemResponse, ()> = {
    ///     // some graceful process
    ///     // ...
    ///     Ok(ItemResponse { item_name: String::from("Item Name") })
    /// };
    ///
    /// ApiBaseResponse::new(result);
    /// ```
    ///
    /// * `result`: processed data
    #[inline]
    pub const fn new(result: Result<T, E>) -> Self {
        Self(result)
    }
}

impl<T, E> ApiMessageResponse<T, E> {
    /// const message response
    ///
    /// ```rust
    /// # use api_presentation::helper::api_response::ApiMessageResponse;
    /// struct ItemResponse {
    ///     item_name: String,
    /// }
    ///
    /// let result: Result<ItemResponse, ()> = {
    ///     // some graceful process
    ///     // ...
    ///     Ok(ItemResponse { item_name: String::from("Item Name") })
    /// };
    ///
    /// ApiMessageResponse::message("Successfully get item", result);
    ///
    /// ```
    /// * `message`: success message
    /// * `result`: processed data
    #[inline]
    pub fn message(message: &'static str, result: Result<T, E>) -> Self {
        match result {
            Ok(t) => Self(Ok((message, t))),
            Err(e) => Self(Err(e)),
        }
    }
}

impl<T: IntoSuccessResponse, E: IntoFailureResponse> IntoResponse for ApiBaseResponse<T, E> {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            Ok(ok) => ok.into_success_response().into_response(),
            Err(err) => err.into_failure_response().into_response(),
        }
    }
}
