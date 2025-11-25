use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use tracing::{Level, event};

use crate::db::DB;
use crate::query::Query;

pub async fn graphql_handler(graphql_request: GraphQLRequest) -> GraphQLResponse {
    event!(Level::INFO, "got a request");

    let query = Query { db: DB {} };
    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(graphql_request.into_inner()).await;

    return res.into();
}
