use crate::service::User;
use async_graphql::{Context, Object};
use sqlx::Pool;
use tracing::{Level, event};

pub struct Query {}

#[Object]
impl Query {
    async fn get_user(&self, ctx: &Context<'_>, id: String) -> Option<User> {
        let pool = ctx
            .data::<Pool<sqlx::Any>>()
            .expect("Database pool unavailable.");

        event!(Level::INFO, "{}", id);
        event!(Level::INFO, "DB pool: {:?}", pool);

        // Query database for user, given the provided ID.
        Some(User::mock())
    }

    async fn get_users(&self, ctx: &Context<'_>) -> Vec<User> {
        let pool = ctx
            .data::<Pool<sqlx::Any>>()
            .expect("Database pool unavailable.");

        event!(Level::INFO, "DB pool: {:?}", pool);

        // Query database for all users.
        vec![User::mock()]
    }

    async fn get_users_by_filter(&self, ctx: &Context<'_>, country: Option<String>) -> Vec<User> {
        let pool = ctx
            .data::<Pool<sqlx::Any>>()
            .expect("Database pool unavailable.");

        event!(Level::INFO, "{:?}", country);
        event!(Level::INFO, "DB pool: {:?}", pool);

        // Query database for users, given a country to filter on.
        vec![User::mock()]
    }
}
