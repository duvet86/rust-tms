use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use tsm::MyAuth;

pub async fn login_handler(State(auth): State<MyAuth>) -> impl IntoResponse {
    Redirect::to(auth.code_flow.authorize_url.as_ref())
}
