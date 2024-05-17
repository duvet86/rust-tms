use anyhow::Context;
use async_session::{serde_json, MemoryStore, Session, SessionStore};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use http::{header::SET_COOKIE, HeaderMap};
use oauth2::TokenResponse;
use url::Url;

use crate::application::{
    models::User,
    utils::{
        app_state::{AppState, MyAuth},
        http_utils::{AppError, AuthRequest, COOKIE_NAME},
    },
};

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/authorized", get(authorized_handler))
}

async fn authorized_handler(
    Query(query): Query<AuthRequest>,
    State(store): State<MemoryStore>,
    State(auth): State<MyAuth>,
) -> Result<impl IntoResponse, AppError> {
    let token: oauth2::StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    > = auth
        .code_flow
        .exchange(
            azure_core::new_http_client(),
            oauth2::AuthorizationCode::new(query.code.clone()),
        )
        .await
        .unwrap();

    let url = Url::parse("https://graph.microsoft.com/v1.0/me")?;

    let text = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().secret()),
        )
        .send()
        .await?
        .text()
        .await?;

    let user: User = serde_json::from_str(&text).unwrap();

    println!("\n\nresp {user:?}");

    // Create a new session filled with user data
    let mut session = Session::new();
    session
        .insert("user", user)
        .context("failed in inserting serialized value into session")?;

    // // Store session and get corresponding cookie
    let cookie = store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}
