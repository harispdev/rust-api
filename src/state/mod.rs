use crate::models::User;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state containing shared data
#[derive(Debug, Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<u32, User>>>,
    pub start_time: std::time::Instant,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            start_time: std::time::Instant::now(),
        }
    }

    /// Get the application uptime in seconds
    pub fn uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}