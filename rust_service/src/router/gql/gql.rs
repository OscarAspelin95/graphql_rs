use crate::state::State as AppState;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use tracing::{Level, event};

pub async fn graphql_handler(
    State(state): State<AppState>,
    graphql_request: GraphQLRequest,
) -> GraphQLResponse {
    event!(Level::INFO, "got a request");

    let res = state.gql_schema.execute(graphql_request.into_inner()).await;

    res.into()
}
