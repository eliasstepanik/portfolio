// Configuration management
// This module can be expanded to handle more complex configuration needs

#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_addr: std::env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
        }
    }
}
