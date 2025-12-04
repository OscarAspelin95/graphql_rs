use crate::context::{create_context, create_schema, Context, Schema};
use crate::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub schema: Arc<Schema>,
    pub context: Context,
}

pub async fn get_state(cfg: &Config) -> State {
    let schema = Arc::new(create_schema());
    let context = create_context(cfg).await;

    State { schema, context }
}
