mod config;
mod errors;
mod handlers;
mod models;
mod routes;
mod state;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use config::Config;
use routes::create_router;
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
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

    // Create application state
    let state = AppState::new();

    // Create the router
    let app = create_router(state);

    // Start the server
    let address = config.server_address();
    let listener = tokio::net::TcpListener::bind(&address).await?;
    
    info!("🚀 Server running on http://{}", address);
    info!("📚 Health check available at http://{}/health", address);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}