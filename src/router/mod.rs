use axum::Router;

mod gql;

pub fn get_routes() -> Router {
    Router::new().merge(gql::routes())
}
