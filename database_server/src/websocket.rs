use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::{sync::broadcast, task};
use log::warn;

use crate::models::{Node, Edge, GraphData, NodeMetadata, NodeDetailsRequest, WebSocketMessage, DatabaseResponse};
use arma3_database::{DatabaseManager, GraphQueryEngine};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State((tx, db)): axum::extract::State<(Arc<broadcast::Sender<String>>, Arc<DatabaseManager>)>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, tx, db))
}

async fn handle_socket(socket: WebSocket, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    // Generate a unique connection ID for logging purposes
    let connection_id = uuid::Uuid::new_v4();
    println!("WebSocket client connected: {}", connection_id);

    // Subscribe to the broadcast channel
    let mut rx = tx.subscribe();

    // Split the socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will receive broadcast messages and send them to this client
    let connection_id_clone = connection_id;
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Convert the string to axum's Message::Text
            if let Err(e) = sender.send(Message::Text(msg.into())).await {
                warn!("Error sending message to client {}: {}", connection_id_clone, e);
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
                    println!("Received message from client {}: action={}", connection_id, ws_msg.action);
                    handle_message(ws_msg, tx_clone.clone(), db_clone.clone()).await;
                }
            }
        }
        
        // If we break out of the loop, the client has disconnected
        println!("WebSocket client disconnected: {}", connection_id);
    });

    // If any of the tasks exit, abort the other one too
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
            println!("WebSocket client disconnected (sender task ended): {}", connection_id);
        },
        _ = (&mut recv_task) => {
            send_task.abort();
            // We already logged the disconnection in the receiver task
        },
    }
}

async fn handle_message(ws_msg: WebSocketMessage, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    match ws_msg.action.as_str() {
        "get_graph" => {
            handle_get_graph(tx, db).await;
        }
        "get_node_details" => {
            if let Some(data) = ws_msg.data {
                if let Ok(request) = serde_json::from_value::<NodeDetailsRequest>(data) {
                    handle_node_details(request, tx, db).await;
                }
            }
        }
        _ => {
            eprintln!("Unknown action: {}", ws_msg.action);
        }
    }
}

async fn handle_get_graph(tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    println!("Starting fast graph data retrieval");
    let start_time = std::time::Instant::now();
    
    let response = task::spawn_blocking(move || {
        let engine = GraphQueryEngine::new(&db);
        let query_start = std::time::Instant::now();
        
        match engine.get_full_graph_fast() {
            Ok(raw_data) => {
                let query_duration = query_start.elapsed();
                println!("Database query took: {:?}", query_duration);
                
                let conversion_start = std::time::Instant::now();
                
                // Convert raw data to websocket format
                let nodes = raw_data.nodes.into_iter()
                    .map(|id| Node {
                        id: id.clone(),
                        name: Some(id),
                        color: None,
                    })
                    .collect::<Vec<_>>();

                let edges = raw_data.edges.into_iter()
                    .map(|(source, target)| Edge {
                        source,
                        target,
                        color: None,
                    })
                    .collect::<Vec<_>>();

                let graph_data = GraphData { nodes, edges };
                
                let conversion_duration = conversion_start.elapsed();
                println!("Data conversion took: {:?}", conversion_duration);

                let serialize_start = std::time::Instant::now();
                let ws_response = WebSocketMessage {
                    action: "graph_data".to_string(),
                    data: Some(serde_json::to_value(graph_data).unwrap()),
                };
                let response = serde_json::to_string(&ws_response).unwrap();
                
                let serialize_duration = serialize_start.elapsed();
                println!("JSON serialization took: {:?}", serialize_duration);
                
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
    println!("Total graph data processing took: {:?}", total_duration);
    
    let _ = tx.send(response);
}

async fn handle_node_details(request: NodeDetailsRequest, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    println!("Retrieving details for node: {}", request.node_id);
    
    let response = task::spawn_blocking(move || {
        let class_repo = arma3_database::queries::class_repository::ClassRepository::new(&db);
        
        match class_repo.get(&request.node_id) {
            Ok(Some(class)) => {
                // Get source path if available
                let source_path = if let Some(idx) = class.source_file_index {
                    class_repo.get_source_path(idx).unwrap_or(None)
                } else {
                    None
                };

                let metadata = NodeMetadata {
                    id: class.id,
                    parent_id: class.parent_id,
                    container_class: class.container_class,
                    source_path,
                    properties: class.properties,
                };

                let ws_response = WebSocketMessage {
                    action: "node_details".to_string(),
                    data: Some(serde_json::to_value(DatabaseResponse {
                        success: true,
                        data: Some(serde_json::to_value(metadata).unwrap()),
                        error: None,
                    }).unwrap()),
                };
                serde_json::to_string(&ws_response).unwrap()
            },
            Ok(None) => {
                let error_response = WebSocketMessage {
                    action: "node_details".to_string(),
                    data: Some(serde_json::to_value(DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some(format!("Node not found: {}", request.node_id)),
                    }).unwrap()),
                };
                serde_json::to_string(&error_response).unwrap()
            },
            Err(e) => {
                let error_response = WebSocketMessage {
                    action: "node_details".to_string(),
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
            action: "node_details".to_string(),
            data: Some(serde_json::to_value(DatabaseResponse {
                success: false,
                data: None,
                error: Some(format!("Task error: {}", e)),
            }).unwrap()),
        };
        serde_json::to_string(&error_response).unwrap()
    });

    let _ = tx.send(response);
}