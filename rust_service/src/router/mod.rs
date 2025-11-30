use crate::state::State;
use axum::Router;
use tower_http::cors::*;

mod gql;
mod ping;

pub fn get_routes(state: State) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        .merge(ping::routes())
        .merge(gql::routes())
        .layer(cors)
        .with_state(state)
}
