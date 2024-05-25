use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::application::utils::{app_state::AppState, http_utils::Claims};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index_handler))
}

async fn index_handler(_: Claims) -> impl IntoResponse {
    Html("Hey. You're logged in!\nYou may now access <a href='/protected'>Protected</a>.\nLog out with <a href='/logout'>Logout</a>.")
}
