use axum::response::{Html, IntoResponse};

use crate::models::user::User;

pub async fn protected_handler(user: User) -> impl IntoResponse {
    Html(format!(
        "Welcome to the protected area :)<br />Here's your info:<br />{user:?}"
    ))
}
