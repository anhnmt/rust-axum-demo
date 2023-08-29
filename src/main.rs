use std::{
    error::Error,
    net::SocketAddr,
    time::Duration
};

use axum::{
    response::Html,
    Router,
    routing::get,
    extract::State,
    http::StatusCode
};
use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Hello, world!");

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:123456aA@@localhost:5432?sslmode=disable".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/ahihi", get(using_connection_pool_extractor))
        .with_state(pool);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}