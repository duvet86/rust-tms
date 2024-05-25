use axum::{
    async_trait,
    extract::FromRequestParts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use dotenvy::var;
use http::{request::Parts, StatusCode};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub oid: String,
    pub name: String,
    pub sub: String,
    pub exp: usize,
}

pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let tenant_id = var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
        let audience = var("AUDIENDE").expect("Missing AUDIENDE environment variable.");

        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError)?;

        let metadata = decode_header(bearer.token()).unwrap();

        let url = Url::parse(&format!(
            "https://login.microsoftonline.com/{}/discovery/v2.0/keys",
            tenant_id
        ));

        let resp = reqwest::Client::new()
            .get(url.unwrap())
            .send()
            .await
            .map_err(|_| AuthError)?
            .text()
            .await
            .map_err(|_| AuthError)?;

        let keys: serde_json::Value = serde_json::from_str(&resp).map_err(|_| AuthError)?;

        let key = keys["keys"]
            .as_array()
            .unwrap()
            .iter()
            .find(|&k| &k["kid"] == metadata.kid.as_ref().unwrap())
            .unwrap();

        let mut validation = Validation::new(Algorithm::RS256);

        validation.set_audience(&[audience]);
        validation.set_issuer(&[format!(
            "https://login.microsoftonline.com/{}/v2.0",
            tenant_id
        )]);

        let pub_key = key["x5c"]
            .as_array()
            .unwrap()
            .first()
            .unwrap()
            .as_str()
            .unwrap();

        let cert = format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----",
            pub_key
        );

        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_rsa_pem(cert.as_bytes()).unwrap(),
            &validation,
        )
        .map_err(|_| AuthError)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
