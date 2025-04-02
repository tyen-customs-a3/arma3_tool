use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::{sync::broadcast, task};

use crate::models::graph::{GraphData, GraphConfig};
use crate::models::websocket::{WebSocketMessage, GenerateRequest, DatabaseQuery, QueryType, ClassHierarchyRequest, ClassImpactRequest, DatabaseResponse};
use crate::services::graph_generator::generate_graph;
use arma3_database::{DatabaseManager, GraphQueryEngine, MissionRepository};

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
        "generate" => {
            if let Some(data) = ws_msg.data {
                if let Ok(request) = serde_json::from_value::<GenerateRequest>(data) {
                    handle_generate(request, tx).await;
                }
            }
        }
        "database_query" => {
            if let Some(data) = ws_msg.data {
                if let Ok(query) = serde_json::from_value::<DatabaseQuery>(data) {
                    handle_database_query(query, tx, db).await;
                }
            }
        }
        _ => {
            eprintln!("Unknown action: {}", ws_msg.action);
        }
    }
}

async fn handle_database_query(query: DatabaseQuery, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    let response = task::spawn_blocking(move || {
        match query.query_type {
            QueryType::GetClassHierarchy => {
                if let Some(params) = query.parameters {
                    if let Ok(request) = serde_json::from_value::<ClassHierarchyRequest>(params) {
                        let engine = GraphQueryEngine::new(&db);
                        match engine.build_class_hierarchy_graph(request.root_class.as_deref(), request.max_depth) {
                            Ok(graph_data) => DatabaseResponse {
                                success: true,
                                data: Some(serde_json::to_value(graph_data).unwrap()),
                                error: None,
                            },
                            Err(e) => DatabaseResponse {
                                success: false,
                                data: None,
                                error: Some(e.to_string()),
                            },
                        }
                    } else {
                        DatabaseResponse {
                            success: false,
                            data: None,
                            error: Some("Invalid parameters for class hierarchy query".to_string()),
                        }
                    }
                } else {
                    DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some("Missing parameters for class hierarchy query".to_string()),
                    }
                }
            }
            QueryType::GetPboDependencies => {
                let engine = GraphQueryEngine::new(&db);
                match engine.build_pbo_dependency_graph() {
                    Ok(graph_data) => DatabaseResponse {
                        success: true,
                        data: Some(serde_json::to_value(graph_data).unwrap()),
                        error: None,
                    },
                    Err(e) => DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                    },
                }
            }
            QueryType::GetMissionDependencies => {
                let repo = MissionRepository::new(&db);
                match repo.get_all_dependencies() {
                    Ok(dependencies) => DatabaseResponse {
                        success: true,
                        data: Some(serde_json::to_value(dependencies).unwrap()),
                        error: None,
                    },
                    Err(e) => DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                    },
                }
            }
            QueryType::GetClassImpact => {
                if let Some(params) = query.parameters {
                    if let Ok(request) = serde_json::from_value::<ClassImpactRequest>(params) {
                        let engine = GraphQueryEngine::new(&db);
                        match engine.impact_analysis(&request.classes_to_remove) {
                            Ok(impact) => DatabaseResponse {
                                success: true,
                                data: Some(serde_json::to_value(impact).unwrap()),
                                error: None,
                            },
                            Err(e) => DatabaseResponse {
                                success: false,
                                data: None,
                                error: Some(e.to_string()),
                            },
                        }
                    } else {
                        DatabaseResponse {
                            success: false,
                            data: None,
                            error: Some("Invalid parameters for class impact query".to_string()),
                        }
                    }
                } else {
                    DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some("Missing parameters for class impact query".to_string()),
                    }
                }
            }
            QueryType::GetDatabaseStats => {
                match db.get_stats() {
                    Ok(stats) => DatabaseResponse {
                        success: true,
                        data: Some(serde_json::to_value(stats).unwrap()),
                        error: None,
                    },
                    Err(e) => DatabaseResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                    },
                }
            }
        }
    }).await.unwrap_or_else(|e| {
        DatabaseResponse {
            success: false,
            data: None,
            error: Some(format!("Task error: {}", e)),
        }
    });

    let ws_response = WebSocketMessage {
        action: "database_response".to_string(),
        data: Some(serde_json::to_value(response).unwrap()),
    };

    let json = serde_json::to_string(&ws_response).unwrap();
    let _ = tx.send(json);
}

async fn handle_generate(request: GenerateRequest, tx: Arc<broadcast::Sender<String>>) {
    let graph_data = task::spawn_blocking(move || {
        generate_graph_from_request(&request)
    }).await.unwrap_or_else(|e| {
        eprintln!("Error generating graph: {}", e);
        GraphData { nodes: vec![], edges: vec![] }
    });
    
    broadcast_graph_data(graph_data, tx).await;
}

async fn broadcast_graph_data(graph_data: GraphData, tx: Arc<broadcast::Sender<String>>) {
    let response = WebSocketMessage {
        action: "graph_data".to_string(),
        data: Some(serde_json::to_value(&graph_data).unwrap()),
    };
    
    let json = serde_json::to_string(&response).unwrap();
    let _ = tx.send(json);
}

fn generate_graph_from_request(request: &GenerateRequest) -> GraphData {
    println!("Generating tree structure with {} nodes", request.node_count);
    
    let config = GraphConfig {
        node_count: request.node_count,
    };
    
    generate_graph(config)
} 