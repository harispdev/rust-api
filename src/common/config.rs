use serde::{Deserialize, Serialize};
use std::env;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub session: SessionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

impl DatabaseConfig {
    /// Build the database URL from individual components
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub secret: String,
    pub redis_url: String,
    pub cookie_name: String,
    pub cookie_domain: Option<String>,
    pub cookie_secure: bool,
    pub cookie_same_site: String,
    pub max_age_seconds: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            database: DatabaseConfig {
                host: env::var("DATABASE_HOST").unwrap_or_else(|_| "postgres".to_string()),
                port: env::var("DATABASE_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .unwrap_or(5432),
                database: env::var("DATABASE_NAME").unwrap_or_else(|_| "rust_api".to_string()),
                username: env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string()),
                password: env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "password".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()
                    .unwrap_or(1),
                acquire_timeout_seconds: env::var("DATABASE_ACQUIRE_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                idle_timeout_seconds: env::var("DATABASE_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .unwrap_or(600),
            },
            logging: LoggingConfig {
                level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            },
            session: SessionConfig {
                secret: env::var("SESSION_SECRET")
                    .unwrap_or_else(|_| "your-super-secret-session-key-change-in-production".to_string()),
                redis_url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                cookie_name: env::var("SESSION_COOKIE_NAME")
                    .unwrap_or_else(|_| "connect.sid".to_string()),
                cookie_domain: env::var("SESSION_COOKIE_DOMAIN").ok(),
                cookie_secure: env::var("SESSION_COOKIE_SECURE")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                cookie_same_site: env::var("SESSION_COOKIE_SAME_SITE")
                    .unwrap_or_else(|_| "lax".to_string()),
                max_age_seconds: env::var("SESSION_MAX_AGE_SECONDS")
                    .unwrap_or_else(|_| "86400".to_string()) // 24 hours
                    .parse()
                    .unwrap_or(86400),
            },
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }
    
    /// Get the server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}