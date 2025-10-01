use anyhow::Result;
use sea_orm::{Database as SeaDatabase, DatabaseConnection, ConnectOptions, ConnectionTrait};
use std::time::Duration;
use tracing::{info, log::LevelFilter};

use crate::common::config::DatabaseConfig;

/// Database connection manager
#[derive(Debug, Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    /// Create a new database connection
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        info!("Connecting to database...");
        
        let mut opt = ConnectOptions::new(&config.url());
        opt.max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout_seconds))
            .idle_timeout(Duration::from_secs(config.idle_timeout_seconds))
            .sqlx_logging(true)
            .sqlx_logging_level(LevelFilter::Info);

        let connection = SeaDatabase::connect(opt).await?;

        // Test the connection
        connection.execute_unprepared("SELECT 1").await?;

        info!("Database connection established successfully");
        Ok(Self { connection })
    }

    /// Get the database connection
    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    /// Health check for the database
    pub async fn health_check(&self) -> Result<()> {
        self.connection.execute_unprepared("SELECT 1").await?;
        Ok(())
    }
}
