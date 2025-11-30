use crate::context::get_gql_context;
use crate::{Config, query::Query};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

#[derive(Clone)]
pub struct State {
    pub gql_schema: Schema<Query, EmptyMutation, EmptySubscription>,
}

pub async fn get_state(cfg: &Config) -> State {
    let gql_schema = get_gql_context(cfg).await;

    State {
        gql_schema: gql_schema,
    }
}
