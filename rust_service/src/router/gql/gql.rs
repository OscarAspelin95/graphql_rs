use crate::state::State as AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use juniper::http::GraphQLRequest;
use tracing::{event, Level};

pub async fn graphql_handler(
    State(state): State<AppState>,
    axum::Json(request): axum::Json<GraphQLRequest>,
) -> impl IntoResponse {
    event!(Level::INFO, "got a graphql request");

    let response = request.execute(&state.schema, &state.context).await;

    axum::Json(response)
}
