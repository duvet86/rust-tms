use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::application::{models::user_dto::UserDto, utils::app_state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/protected", get(protected_handler))
}

async fn protected_handler(user: UserDto) -> impl IntoResponse {
    Html(format!(
        "Welcome to the protected area :)<br />Here's your info:<br />{user:?}"
    ))
}
