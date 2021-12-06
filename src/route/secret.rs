use crate::{auth::claim::Claims, configuration::AuthSettings, helper::error_chain_fmt};
use actix_web::{body::AnyBody, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::anyhow;
use jsonwebtoken::{decode, Validation};
use reqwest::StatusCode;

#[derive(thiserror::Error)]
pub enum SecretError {
    #[error("Authenticaion failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Access forbidden")]
    Forbidden(#[source] anyhow::Error),
}

impl std::fmt::Debug for SecretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SecretError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SecretError::AuthError(_) => {
                let response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                response.set_body(AnyBody::Bytes(
                    r#"<p>Unauthorized. <a href="/login">login</a></p>"#.into(),
                ))
            }
            SecretError::Forbidden(_) => HttpResponse::new(StatusCode::FORBIDDEN),
        }
    }
}

pub async fn secret<'a>(req: HttpRequest) -> Result<HttpResponse, SecretError> {
    match req.extensions().get::<Claims>() {
        Some(claims) => Ok(HttpResponse::Ok().body(format!("{} got secret", claims.id.email))),
        None => panic!("Error"),
    }
}
