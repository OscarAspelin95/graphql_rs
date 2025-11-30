pub struct Config {
    pub port: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: std::env::var("RUST_PORT").unwrap_or_else(|_| "3000".into()),
            database_url: std::env::var("DATABASE_URL")
                .expect("Database URL must exist in environment."),
        }
    }
}
