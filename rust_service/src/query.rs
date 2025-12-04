use crate::context::Context;
use crate::service::User;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn get_user(user_id: String, context: &Context) -> FieldResult<User> {
        let pool = &context.db;

        let user_uuid = uuid::Uuid::parse_str(&user_id).map_err(|err| format!("{:?}", err))?;

        let user = sqlx::query_as!(
            User,
            "SELECT id, first_name, last_name, created_at, updated_at FROM public.users WHERE id = $1",
            user_uuid
        ).fetch_one(pool).await?;

        Ok(user)
    }

    async fn get_all_users(context: &Context) -> FieldResult<Vec<User>> {
        let pool = &context.db;

        let users = sqlx::query_as!(
            User,
            "SELECT id, first_name, last_name, created_at, updated_at FROM public.users"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    fn api_version() -> &'static str {
        "1.0"
    }
}
