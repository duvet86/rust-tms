use axum::{response::IntoResponse, routing::get, Router};
use http::StatusCode;

use crate::application::utils::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/401", get(forbidden_handler))
}

async fn forbidden_handler() -> impl IntoResponse {
    (StatusCode::FORBIDDEN, "No credentials")
}
