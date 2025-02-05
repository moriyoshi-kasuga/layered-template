use std::str::FromStr;

use api_domain::{model::HealthCheckModel, repository::health_check::MockHealthCheckRepository};
use chrono::DateTime;

use crate::router::tests::{assert_response, MockTester};

#[tokio::test]
async fn test() {
    let mut tester = MockTester::new();

    let time = DateTime::from_str("2025-02-05T22:49:22.000846+09:00").unwrap();

    let mut health_check = MockHealthCheckRepository::new();

    health_check
        .expect_health_check()
        .returning(move || Box::pin(async move { Ok(HealthCheckModel { server_time: time }) }));

    tester
        .repo()
        .expect_health_check_repository()
        .return_const(health_check);

    let response = tester.server.get("/api/hc").await;
    assert_response(&response, include_str!("./get_health_check.json"));
}
