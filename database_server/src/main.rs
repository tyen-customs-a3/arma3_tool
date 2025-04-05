#![warn(dead_code, unused_imports, unused_variables, deprecated_in_future)]
#![deny(clippy::all)]

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::{cors::CorsLayer, services::ServeDir};
use std::env;

mod models;
mod websocket;

use websocket::ws_handler;
use arma3_database::{DatabaseManager, CacheConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // Initialize tracing with more detail
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Get database path from command line arguments or use default
    let args: Vec<String> = env::args().collect();
    let db_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("data/database.db")
    };

    println!("Loading database from: {:?}", db_path);

    // Initialize database
    let config = CacheConfig::new(db_path, "cache");
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