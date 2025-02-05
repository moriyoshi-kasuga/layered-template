use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = self.to_string();
        serializer.serialize_str(&message)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
