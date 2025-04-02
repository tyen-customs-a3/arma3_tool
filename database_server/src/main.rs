use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod models;
mod handlers;
mod services;

use handlers::websocket::ws_handler;
use arma3_database::{DatabaseManager, CacheConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // Initialize tracing with more detail
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initialize database
    let config = CacheConfig::new(PathBuf::from("data/database.db"), "cache");
    let db = DatabaseManager::with_config(config).expect("Failed to initialize database");
    let db = Arc::new(db);

    // Channel for broadcasting messages to all connected clients
    let (tx, _) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx);

    // Create the router with the HTTP endpoints
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state((tx, db));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
} 