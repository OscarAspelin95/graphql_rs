use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::Router;
use axum::routing::post;

mod db;
mod query;
mod service;

use crate::db::DB;
use crate::query::Query;

async fn graphpl_handler(graphql_request: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db: DB {} };

    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(graphql_request.into_inner()).await;

    return res.into();
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/gql", post(graphpl_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
