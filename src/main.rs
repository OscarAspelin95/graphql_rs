use tracing::{Level, event};
use tracing_subscriber;

use crate::config::Config;

mod config;
mod db;
mod query;
mod router;
mod service;

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt::init();

    //
    let cfg = Config::new();

    // Define a graphql router.
    let app = router::get_routes();

    // Bind to localhost:3000.
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.port))
        .await
        .expect(&format!("Failed to bind to host at {}.", cfg.port));

    // Serve app.
    event!(Level::INFO, "Starting server on port {}...", cfg.port);
    axum::serve(listener, app)
        .await
        .expect("Failed to start server.");
}
