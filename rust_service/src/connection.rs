use crate::Config;
use sqlx::Pool;

/// Get a database pool once on startup. Then, we can acquire connections when we run queries.
pub async fn connect_to_postgres(cfg: &Config) -> Pool<sqlx::Postgres> {
    let pool: Pool<sqlx::Postgres> = sqlx::Pool::connect(&cfg.database_url)
        .await
        .expect("Failed to connect with database pool");

    pool
}
