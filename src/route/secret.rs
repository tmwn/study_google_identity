use crate::{auth::google, configuration::AdminEmails, helper::error_chain_fmt};
use actix_web::{body::AnyBody, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::anyhow;
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

pub async fn secret(
    req: HttpRequest,
    admins: web::Data<AdminEmails>,
) -> Result<HttpResponse, SecretError> {
    // TODO: CSRF prevention.
    let google_jwt = req
        .cookie(google::COOKIE_KEY)
        .ok_or_else(|| SecretError::AuthError(anyhow!("credential not found in cookie")))?;
    let id = google::decode(google_jwt.value())
        .await
        .map_err(SecretError::AuthError)?;
    if !admins.contains(&id.email) {
        return Err(SecretError::Forbidden(anyhow!("use is not admin")));
    }
    Ok(HttpResponse::Ok().finish())
}
