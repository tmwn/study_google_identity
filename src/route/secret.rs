use crate::helper::error_chain_fmt;
use actix_web::{
    body::AnyBody, http::HeaderValue, Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use anyhow::anyhow;
use reqwest::{header, Request, StatusCode};

#[derive(thiserror::Error)]
pub enum SecretError {
    #[error("Authenticaion failed")]
    AuthError(#[source] anyhow::Error),
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
                let response = response.set_body(AnyBody::Bytes(
                    r#"<p>Unauthorized. <a href="/login">login</a></p>"#.into(),
                ));
                response
            }
        }
    }
}

pub async fn secret(req: HttpRequest) -> Result<HttpResponse, SecretError> {
    let cred = req
        .cookie("credential")
        .ok_or(SecretError::AuthError(anyhow!(
            "credential not found in cookie"
        )))?;
    Ok(HttpResponse::Ok().finish())
}
