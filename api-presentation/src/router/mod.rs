use std::sync::Arc;

use api_application::modules::UseCaseModule;
use axum::Extension;

pub mod health_check;

pub type UseCaseExtension<R> = Extension<Arc<UseCaseModule<R>>>;
