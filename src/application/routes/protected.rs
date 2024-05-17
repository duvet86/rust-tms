use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::application::{models::User, utils::app_state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/protected", get(protected_handler))
}

async fn protected_handler(user: User) -> impl IntoResponse {
    Html(format!(
        "Welcome to the protected area :)<br />Here's your info:<br />{user:?}"
    ))
}
