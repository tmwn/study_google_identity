use std::time::SystemTime;

use actix_web::body::AnyBody;
use actix_web::ResponseError;
use actix_web::{cookie::Cookie, http::HeaderValue, web, HttpResponse, Responder};
use jsonwebtoken::{encode, Header};
use reqwest::header;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::auth::claim::Claims;
use crate::auth::google;
use crate::configuration::AuthSettings;
use crate::helper::error_chain_fmt;

const CLIENT_ID: &str = "917732578375-3tpjt3d19phubohmheel57b5hn7g1fl0.apps.googleusercontent.com";

pub async fn login() -> impl Responder {
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

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authenticaion failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Access forbidden")]
    Forbidden(#[source] anyhow::Error),
    #[error("Unexpected error")]
    InternalError(#[source] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LoginError::AuthError(_) => {
                let response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                response.set_body(AnyBody::Bytes(
                    r#"<p>Unauthorized. <a href="/login">login</a></p>"#.into(),
                ))
            }
            LoginError::Forbidden(_) => {
                let response = HttpResponse::new(StatusCode::FORBIDDEN);
                response.set_body(AnyBody::Bytes(r#"<p>Access forbidden.</p>"#.into()))
            }
            LoginError::InternalError(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub async fn login_endpoint(
    web::Form(info): web::Form<LoginRequest>,
    settings: web::Data<AuthSettings>,
) -> Result<HttpResponse, LoginError> {
    let mut res = HttpResponse::SeeOther().body("");
    res.headers_mut()
        .append(header::LOCATION, HeaderValue::from_static("/secret"));

    // Validate the cookie here, and if valid, store the token for our app in cookie.
    let id = google::decode(&info.credential)
        .await
        .map_err(LoginError::AuthError)?;
    if !settings.admin_google_emails.contains(&id.email) {
        return Err(LoginError::Forbidden(anyhow::anyhow!("user is not admin")));
    }

    let current_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| LoginError::InternalError(e.into()))?;
    let exp = current_timestamp.as_secs() as usize + 60;
    let claim = Claims { exp, id };

    // Encode the id with HS256 algorithm.
    let token = encode(&Header::default(), &claim, &settings.encoding_key)
        .map_err(|e| LoginError::InternalError(e.into()))?;
    let cookie = Cookie::build("login_token", token).http_only(true).finish();
    res.add_cookie(&cookie).unwrap();
    Ok(res)
}
