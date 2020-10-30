use reqwest;

mod common;
use common::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let test_app = spawn_app().await;
    let address = test_app.address;
    let url = format!("{}/health_check", &address);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
