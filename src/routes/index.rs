use axum::response::{Html, IntoResponse};

use crate::models::user::User;

pub async fn index_handler(user: User) -> impl IntoResponse {
    Html(format!(
        "Hey '{}'. You're logged in!\nYou may now access <a href='/protected'>Protected</a>.\nLog out with <a href='/logout'>Logout</a>.",
        user.display_name))
}
