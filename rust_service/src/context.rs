use crate::{config::Config, connection::connect_to_postgres, query::Query};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub async fn get_gql_context(cfg: &Config) -> Schema<Query, EmptyMutation, EmptySubscription> {
    let pool = connect_to_postgres(cfg).await;
    let query = Query {};

    let schema = Schema::build(query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish();

    schema
}
