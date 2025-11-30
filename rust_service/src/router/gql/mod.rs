pub mod gql;
use crate::router::gql::gql::graphql_handler;
use crate::state::State;
use axum::{Router, routing::post};

pub fn routes() -> Router<State> {
    Router::new().route("/gql", post(graphql_handler))
}
