use std::env;

use axum::{extract::Request, middleware::Next, response::Response};

use azure_identity::authorization_code_flow::{self, AuthorizationCodeFlow};
use http::{HeaderMap, StatusCode};
use oauth2::{ClientId, ClientSecret};
use url::Url;

pub fn get_code_flow() -> AuthorizationCodeFlow {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:3000/auth/authorized".to_string());

    authorization_code_flow::start(
        client_id,
        Some(client_secret),
        &tenant_id,
        Url::parse(&redirect_url).unwrap(),
        &["openid", "profile", "email"],
    )
}

pub async fn auth_middlewere(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match has_token(&headers) {
        true => {
            let response = next.run(request).await;
            Ok(response)
        }
        false => Err(StatusCode::UNAUTHORIZED),
    }
}

fn has_token(headers: &HeaderMap) -> bool {
    match headers.get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.contains("Bearer "),
            Err(_) => false,
        },
        None => false,
    }
}
