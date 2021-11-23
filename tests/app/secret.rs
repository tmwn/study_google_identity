use reqwest::StatusCode;

use crate::helper::spawn_app;

#[actix_rt::test]
async fn secret_should_fail_if_no_credential() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/secret", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), 401);
}
