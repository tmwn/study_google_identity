use actix_web::{cookie::Cookie, http::HeaderValue, web, HttpRequest, HttpResponse, Responder};
use reqwest::header;
use serde::Deserialize;

use crate::auth::google;

const CLIENT_ID: &str = "917732578375-3tpjt3d19phubohmheel57b5hn7g1fl0.apps.googleusercontent.com";

pub async fn login(req: HttpRequest) -> impl Responder {
    let body = format!(
        r#"
    <html>
    <body>
        <script src="https://accounts.google.com/gsi/client" async defer></script>
        <div id="g_id_onload"
           data-client_id="{}"
           data-login_uri="http://localhost:8080/login"
           data-auto_prompt="false">
        </div>
        <div class="g_id_signin"
           data-type="standard"
           data-size="large"
           data-theme="outline"
           data-text="sign_in_with"
           data-shape="rectangular"
           data-logo_alignment="left">
        </div>
    </body>
  </html>
    "#,
        CLIENT_ID
    );
    HttpResponse::Ok().body(body)
}

#[derive(Deserialize)]
pub struct LoginRequest {
    credential: String,
}

// Stores Google's JWT token in cookie.
// Note: we are not validating the token yet. We will validate it in each handler.
pub async fn login_endpoint(web::Form(info): web::Form<LoginRequest>) -> impl Responder {
    let mut res = HttpResponse::SeeOther().body("");
    res.headers_mut()
        .append(header::LOCATION, HeaderValue::from_static("/secret"));

    let cookie = Cookie::build(google::COOKIE_KEY, info.credential)
        .http_only(true)
        .finish();
    res.add_cookie(&cookie).unwrap();
    return res;
}
