use std::sync::Arc;

use api_application::modules::UseCaseModule;
use axum::Extension;

pub mod health_check;

#[cfg(test)]
pub mod tests;

pub type UseCaseExtension<R> = Extension<Arc<UseCaseModule<R>>>;
