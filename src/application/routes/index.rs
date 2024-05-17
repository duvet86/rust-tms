use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::application::{models::user_dto::UserDto, utils::app_state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index_handler))
}

async fn index_handler(user: UserDto) -> impl IntoResponse {
    Html(format!(
        "Hey '{}'. You're logged in!\nYou may now access <a href='/protected'>Protected</a>.\nLog out with <a href='/logout'>Logout</a>.",
        user.display_name))
}
