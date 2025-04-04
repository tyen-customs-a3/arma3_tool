use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::{sync::broadcast, task};
use log::debug;

use crate::models::graph::{self as ws_graph};
use crate::models::websocket::{WebSocketMessage, DatabaseResponse};
use arma3_database::{DatabaseManager, GraphQueryEngine};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State((tx, db)): axum::extract::State<(Arc<broadcast::Sender<String>>, Arc<DatabaseManager>)>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, tx, db))
}

async fn handle_socket(socket: WebSocket, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    // Subscribe to the broadcast channel
    let mut rx = tx.subscribe();

    // Split the socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will receive broadcast messages and send them to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Convert the string to axum's Message::Text
            if let Err(e) = sender.send(Message::Text(msg.into())).await {
                eprintln!("Error sending message to client: {}", e);
                break;
            }
        }
    });

    // Process incoming messages from this client
    let tx_clone = Arc::clone(&tx);
    let db_clone = Arc::clone(&db);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Parse the incoming WebSocket message
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                    handle_message(ws_msg, tx_clone.clone(), db_clone.clone()).await;
                }
            }
        }
    });

    // If any of the tasks exit, abort the other one too
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

async fn handle_message(ws_msg: WebSocketMessage, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    match ws_msg.action.as_str() {
        "get_graph" => {
            handle_get_graph(tx, db).await;
        }
        _ => {
            eprintln!("Unknown action: {}", ws_msg.action);
        }
    }
}

async fn handle_get_graph(tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    debug!("Starting fast graph data retrieval");
    let start_time = std::time::Instant::now();
    
    let response = task::spawn_blocking(move || {
        let engine = GraphQueryEngine::new(&db);
        let query_start = std::time::Instant::now();
        
        match engine.get_full_graph_fast() {
            Ok(raw_data) => {
                let query_duration = query_start.elapsed();
                debug!("Database query took: {:?}", query_duration);
                
                let conversion_start = std::time::Instant::now();
                
                // Convert raw data to websocket format
                let nodes = raw_data.nodes.into_iter()
                    .map(|id| ws_graph::Node {
                        id: id.clone(),
                        name: Some(id),
                        color: None,
                    })
                    .collect::<Vec<_>>();

                let edges = raw_data.edges.into_iter()
                    .map(|(source, target)| ws_graph::Edge {
                        source,
                        target,
                        color: None,
                    })
                    .collect::<Vec<_>>();

                let graph_data = ws_graph::GraphData { nodes, edges };
                
                let conversion_duration = conversion_start.elapsed();
                debug!("Data conversion took: {:?}", conversion_duration);

                let serialize_start = std::time::Instant::now();
                let ws_response = WebSocketMessage {
                    action: "graph_data".to_string(),
                    data: Some(serde_json::to_value(graph_data).unwrap()),
                };
                let response = serde_json::to_string(&ws_response).unwrap();
                
                let serialize_duration = serialize_start.elapsed();
                debug!("JSON serialization took: {:?}", serialize_duration);
                
                response
            },
            Err(e) => {
                let error_response = WebSocketMessage {
                    action: "graph_data".to_string(),
                    data: Some(serde_json::to_value(DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                    }).unwrap()),
                };
                serde_json::to_string(&error_response).unwrap()
            },
        }
    }).await.unwrap_or_else(|e| {
        let error_response = WebSocketMessage {
            action: "graph_data".to_string(),
            data: Some(serde_json::to_value(DatabaseResponse {
                success: false,
                data: None,
                error: Some(format!("Task error: {}", e)),
            }).unwrap()),
        };
        serde_json::to_string(&error_response).unwrap()
    });

    let total_duration = start_time.elapsed();
    debug!("Total graph data processing took: {:?}", total_duration);
    
    let _ = tx.send(response);
}
