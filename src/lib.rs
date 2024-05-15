use std::env;

use async_session::MemoryStore;
use axum::{
    extract::FromRef,
    response::{IntoResponse, Redirect, Response},
};
use azure_identity::authorization_code_flow::{self, AuthorizationCodeFlow};
use http::StatusCode;
use oauth2::{ClientId, ClientSecret};
use serde::Deserialize;
use url::Url;

pub static COOKIE_NAME: &str = "SESSION";

pub struct AuthRedirect;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/login").into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub auth: MyAuth,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for MyAuth {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}

pub struct MyAuth {
    pub code_flow: AuthorizationCodeFlow,
}

// pkce_code_verifier doesn't support clone so we have to recreate the entire AuthorizationCodeFlow.
impl Clone for MyAuth {
    fn clone(&self) -> Self {
        MyAuth {
            code_flow: AuthorizationCodeFlow {
                authorize_url: self.code_flow.authorize_url.clone(),
                client: self.code_flow.client.clone(),
                csrf_state: oauth2::CsrfToken::new(self.code_flow.csrf_state.secret().clone()),
                pkce_code_verifier: oauth2::PkceCodeVerifier::new(
                    self.code_flow.pkce_code_verifier.secret().clone(),
                ),
            },
        }
    }
}

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
