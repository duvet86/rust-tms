use axum::{async_trait, extract::FromRequestParts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use dotenvy::var;
use http::request::Parts;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use url::Url;

use super::utils::http_utils::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequireAuth {
    pub oid: String,
    pub name: String,
    pub preferred_username: String,
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
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
        let token_data = decode::<RequireAuth>(
            bearer.token(),
            &DecodingKey::from_rsa_pem(cert.as_bytes()).unwrap(),
            &validation,
        )
        .map_err(|_| AuthError)?;

        Ok(token_data.claims)
    }
}
