use std::sync::Arc;

use jsonwebtoken::{encode, Header};
use reqwest::cookie::Jar;
use study_google_auth::{
    auth::{claim::Claims, google::GoogleId},
    route::login,
};

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
async fn secret_should_fail_if_user_is_not_admin() {
    let app = spawn_app().await;

    let claims = Claims {
        exp: 60 * 60 * 24 * 365 * 1000, // 1000 years
        id: GoogleId {
            email: "foo@example.com".to_string(),
        },
    };
    let token = encode(&Header::default(), &claims, &app.encoding_key).unwrap();

    let cookies = Jar::default();
    cookies.add_cookie_str(
        &format!("{}={}", login::COOKIE_KEY, token),
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

    assert_eq!(response.status(), 403);
}
