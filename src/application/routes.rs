use anyhow::Context;
use axum::{middleware::from_extractor, Router};
use http::{HeaderValue, Method};
use listenfd::ListenFd;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod customers;
mod forbidden;
mod index;
mod me;

use super::{auth::RequireAuth, utils::app_state::AppState};

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let app = create_app(db);

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000")
            .await
            .context("failed to bind TcpListener")
            .unwrap(),
    };

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")
            .unwrap()
    );

    axum::serve(listener, app)
        .await
        .context("failed to serve API")
}

fn create_app(db: PgPool) -> Router {
    let app_state = AppState { db_pool: db };

    let api_routes = Router::new()
        .merge(customers::router())
        .merge(me::router())
        .route_layer(from_extractor::<RequireAuth>());

    Router::new()
        .merge(index::router())
        .merge(forbidden::router())
        .nest("/v1/api", api_routes)
        .with_state(app_state)
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
}
