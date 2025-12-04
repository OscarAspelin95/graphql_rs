use crate::{config::Config, connection::connect_to_postgres, query::Query};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Context {
    pub db: Pool<Postgres>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

pub async fn create_context(cfg: &Config) -> Context {
    let pool: Pool<Postgres> = connect_to_postgres(cfg).await;
    Context { db: pool }
}
