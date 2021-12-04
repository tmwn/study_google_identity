use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use jsonwebtoken::{decode_header, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use reqwest::{header::CACHE_CONTROL, Response};
use serde::Deserialize;
use tokio::sync::Mutex;

use super::cache::Cache;

pub const COOKIE_KEY: &str = "google_jwt";

// Decode Google-provided JWT token with validation.
pub async fn decode(token: &str) -> anyhow::Result<GoogleId> {
    let header = decode_header(token).context("failed to decode header")?;
    let kid = header
        .kid
        .ok_or_else(|| anyhow!("token doesn't contains kid field"))?;
    let key = get_key(&kid)
        .await
        .context(format!("failed to get Google public key with kid={}", kid))?;
    let data: TokenData<GoogleId> = jsonwebtoken::decode(
        token,
        &DecodingKey::from_rsa_components(&key.n, &key.e),
        &Validation::new(jsonwebtoken::Algorithm::from_str(&key.alg)?),
    )
    .context("failed to decode token")?;
    Ok(data.claims)
}
#[derive(Deserialize, Clone, Debug)]
pub struct JWK {
    kid: String,
    pub n: String,
    pub e: String,
    pub alg: String,
    kty: String,
}

#[derive(Deserialize)]
pub struct GoogleId {
    pub email: String,
}

// Get the public key from Google auth server.
pub async fn get_key(kid: &str) -> anyhow::Result<JWK> {
    for x in GoogleJWKS::get().await?.keys {
        if x.kid != kid {
            continue;
        }
        if x.alg != "RS256" {
            bail!("unexpected algorithm {}", x.alg);
        }
        return Ok(x);
    }
    bail!("{} not found", kid)
}

#[derive(Deserialize, Clone)]
struct GoogleJWKS {
    keys: Vec<JWK>,
}

impl GoogleJWKS {
    async fn get() -> anyhow::Result<Self> {
        let mut cache = JWKS_CACHE.lock().await;
        let now = std::time::Instant::now();
        if let Some(keys) = cache.get(&now) {
            return Ok(keys.clone());
        }
        let (res, age) = Self::fetch().await?;
        cache.set(res.clone(), now + age);
        Ok(res)
    }
    async fn fetch() -> anyhow::Result<(Self, std::time::Duration)> {
        let response = reqwest::get(JWKS_URI).await?;
        let age = max_age(&response)?;
        let jwks: Self = response.json().await.map_err(|e| anyhow!("{}", e))?;
        Ok((jwks, age))
    }
}

static JWKS_CACHE: Lazy<Mutex<Cache<GoogleJWKS>>> = Lazy::new(|| Mutex::new(Cache::new()));

const JWKS_URI: &str = "https://www.googleapis.com/oauth2/v3/certs";

// Extract max-age header value.
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control
fn max_age(resp: &Response) -> anyhow::Result<std::time::Duration> {
    let cc = resp
        .headers()
        .get(CACHE_CONTROL)
        .ok_or_else(|| anyhow!("no {} header", CACHE_CONTROL))?;
    let age_sec: u64 = cc
        .to_str()?
        .split(", ")
        .filter_map(|x| x.strip_prefix("max-age="))
        .next()
        .ok_or_else(|| anyhow!("max-age key not found"))?
        .parse()?;
    Ok(std::time::Duration::from_secs(age_sec))
}

#[cfg(test)]
mod tests {
    use super::get_key;

    #[actix_rt::test]
    async fn google_jwks() {
        // TODO: use fake backend.
        let x = get_key("9341abc4092b6fc038e403c91022dd3e44539b56")
            .await
            .unwrap();
        assert_eq!(x.alg, "RS256");
    }
}
