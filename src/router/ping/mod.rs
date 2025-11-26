pub mod ping;

use crate::router::ping::ping::ping_handler;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    return Router::new().route("/ping", post(ping_handler));
}
