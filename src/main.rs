mod common;
mod modules;
mod routes;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use dotenvy::dotenv;

use common::{Config, Database, AppState, session::create_session_layer};
use routes::create_router;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Load configuration
    let config = Config::from_env();
    
    // Initialize tracing with proper configuration
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .compact()
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Rust API server...");
    info!("Configuration: {:?}", config);

    // Initialize database
    let database = Database::new(&config.database).await?;

    // Test database connection
    database.health_check().await?;
    info!("✅ Database connection verified");

    // Create application state
    let state = AppState::new(database);

    // Create session layer
    let session_layer = create_session_layer(&config.session).await;
    
    // Create the router with session middleware
    let app = create_router(state)
        .layer(session_layer);

    // Start the server
    let address = config.server_address();
    let listener = tokio::net::TcpListener::bind(&address).await?;
    
    info!("🚀 Server running on http://{}", address);
    info!("📚 Health check available at http://{}/health", address);
    info!("🗄️  Database connected and migrations applied");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}