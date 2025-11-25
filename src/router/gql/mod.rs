pub mod gql;
use crate::router::gql::gql::graphql_handler;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    return Router::new().route("/gql", post(graphql_handler));
}
