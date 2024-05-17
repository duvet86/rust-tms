pub mod app_state;
pub mod http_utils;

use std::env;

use azure_identity::authorization_code_flow::{self, AuthorizationCodeFlow};
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
