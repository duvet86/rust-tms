use anyhow::Context;
use axum::{middleware, Router};
use listenfd::ListenFd;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod customers;
mod forbidden;
mod index;

use super::{auth::auth_middlewere, utils::app_state::AppState};

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
    // let code_flow = get_code_flow();

    // let store = MemoryStore::new();
    let app_state = AppState { db_pool: db };

    let api_routes = Router::new()
        .merge(customers::router())
        .layer(middleware::from_fn(auth_middlewere));

    Router::new()
        .merge(index::router())
        .merge(forbidden::router())
        .nest("/api", api_routes)
        .with_state(app_state)
}
