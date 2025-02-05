use std::sync::Arc;

use api_application::modules::UseCaseModule;
use axum::Extension;

pub mod helper;
pub mod model;
pub mod router;
pub mod startup;

pub type UseCaseExtension<R> = Extension<Arc<UseCaseModule<R>>>;
