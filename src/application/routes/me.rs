use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;

use crate::application::auth::RequireAuth;
use crate::application::utils::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(me_handler))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Me {
    id: String,
    display_name: String,
    username: String,
    roles: Vec<String>,
}

async fn me_handler(claims: RequireAuth) -> impl IntoResponse {
    Json(Me {
        id: claims.oid,
        display_name: claims.name,
        username: claims.preferred_username,
        roles: claims.roles,
    })
}
