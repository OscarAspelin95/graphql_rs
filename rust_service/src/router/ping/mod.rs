pub mod ping;

use crate::{router::ping::ping::ping_handler, state::State};
use axum::{Router, routing::post};

pub fn routes() -> Router<State> {
    Router::new().route("/ping", post(ping_handler))
}
