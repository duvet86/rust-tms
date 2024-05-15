mod models;
mod routes;

use anyhow::Context;
use async_session::MemoryStore;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tsm::{get_code_flow, AppState, MyAuth};

use crate::routes::{
    authorized::authorized_handler, index::index_handler, login::login_handler,
    logout::logout_handler, protected::protected_handler,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = dotenvy::var("DATABASE_URL")
        // The error from `var()` doesn't mention the environment variable.
        .context("DATABASE_URL must be set")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;

    sqlx::migrate!().run(&db).await?;

    let code_flow = get_code_flow();

    let store = MemoryStore::new();
    let app_state = AppState {
        store,
        auth: MyAuth { code_flow },
    };
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/auth/login", get(login_handler))
        .route("/auth/authorized", get(authorized_handler))
        .route("/protected", get(protected_handler))
        .route("/logout", get(logout_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

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
