use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

use crate::application::utils::app_state::{AppState, MyAuth};

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/login", get(login_handler))
}

async fn login_handler(State(auth): State<MyAuth>) -> impl IntoResponse {
    Redirect::to(auth.code_flow.authorize_url.as_ref())
}
