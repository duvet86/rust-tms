use anyhow::Context;
use async_session::{MemoryStore, SessionStore};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::{headers, TypedHeader};

use crate::application::utils::{
    app_state::AppState,
    http_utils::{AppError, COOKIE_NAME},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", get(logout_handler))
}

async fn logout_handler(
    State(store): State<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(s) => s,
        // No session active, just redirect
        None => return Ok(Redirect::to("/")),
    };

    store
        .destroy_session(session)
        .await
        .context("failed to destroy session")?;

    Ok(Redirect::to("/"))
}
