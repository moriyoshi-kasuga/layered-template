pub mod health_check;

use std::sync::Arc;

use api_application::modules::UseCaseModule;
use api_domain::modules::MockRepositoryModule;
use axum::http::StatusCode;
use axum_test::{TestResponse, TestServer};

use crate::startup::router::create_router;

pub struct MockTester {
    repo: Arc<MockRepositoryModule>,
    pub server: TestServer,
}

impl Default for MockTester {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTester {
    pub fn new() -> Self {
        let repo = Arc::new(MockRepositoryModule::new());
        let use_case = UseCaseModule::new(Arc::clone(&repo));
        let use_case = Arc::new(use_case);
        let router = create_router(use_case);

        let server = TestServer::new(router).unwrap();

        Self { repo, server }
    }

    pub fn repo(&mut self) -> &mut MockRepositoryModule {
        mock_mutable(&mut self.repo)
    }
}

impl Drop for MockTester {
    fn drop(&mut self) {
        self.repo().checkpoint();
    }
}

#[inline]
#[track_caller]
pub fn to_serde_value(json: &str) -> serde_json::Value {
    serde_json::from_str::<serde_json::Value>(json).unwrap()
}

#[track_caller]
pub fn assert_response(response: &TestResponse, json: &str) {
    let value = to_serde_value(json);
    let status_code_value = value.get("status").unwrap();
    let status_code_u16 = status_code_value.as_u64().unwrap() as u16;
    let status_code = StatusCode::from_u16(status_code_u16).unwrap();
    response.assert_status(status_code);
    response.assert_json(&value);
}

// test 時には Arc 使用中にこの関数を使用しなければ safe です
pub fn mock_mutable<Mock>(mock: &mut Arc<Mock>) -> &mut Mock {
    let ptr: *mut Mock = Arc::as_ptr(mock) as *mut Mock;
    unsafe { ptr.as_mut().unwrap() }
}
