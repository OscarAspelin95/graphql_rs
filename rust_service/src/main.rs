use crate::config::Config;
use dotenv;
use tracing::{Level, event};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

mod config;
mod connection;
mod context;
mod query;
mod router;
mod service;
mod state;

#[tokio::main]
async fn main() {
    // For development
    let _ = dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let cfg = Config::new();
    let state = state::get_state(&cfg).await;
    let app = router::get_routes(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.port))
        .await
        .expect(&format!("Failed to bind to host at {}.", cfg.port));

    // Serve app.
    event!(Level::INFO, "Starting server on port {}...", cfg.port);
    axum::serve(listener, app)
        .await
        .expect("Failed to start server.");
}
