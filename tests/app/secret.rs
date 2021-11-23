use std::sync::Arc;

use reqwest::cookie::Jar;
use study_google_auth::auth::google;

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

#[actix_rt::test]
async fn secret_should_fail_if_credential_is_invalid() {
    let app = spawn_app().await;

    let cookies = Jar::default();
    cookies.add_cookie_str(
        &format!("{}=foobar", google::COOKIE_KEY),
        &app.address.parse().unwrap(),
    );

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(cookies))
        .build()
        .expect("failed to build client");
    let response = client
        .get(&format!("{}/secret", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), 401);
}
