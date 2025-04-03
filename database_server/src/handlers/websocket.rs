use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::{sync::broadcast, task};
use serde::Deserialize;
use log::debug;

use crate::models::graph::{self as ws_graph};
use crate::models::websocket::{WebSocketMessage, DatabaseQuery, QueryType, ClassHierarchyRequest, ClassImpactRequest, DatabaseResponse};
use arma3_database::{DatabaseManager, GraphQueryEngine, MissionRepository};

#[derive(Debug, Deserialize)]
struct GraphQueryParams {
    exclude_source_patterns: Option<Vec<String>>,
    max_depth: Option<i32>,
    root_class: Option<String>,
}

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
        "get_full_graph" => {
            if let Some(data) = ws_msg.data {
                if let Ok(params) = serde_json::from_value::<GraphQueryParams>(data) {
                    handle_get_full_graph(params, tx, db).await;
                } else {
                    // If no valid params provided, use defaults
                    handle_get_full_graph(GraphQueryParams {
                        exclude_source_patterns: None,
                        max_depth: Some(100),
                        root_class: None,
                    }, tx, db).await;
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

async fn handle_get_full_graph(params: GraphQueryParams, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    debug!("Received graph query params: {:?}", params);
    let response = task::spawn_blocking(move || {
        let engine = GraphQueryEngine::new(&db);
        let result = engine.build_class_hierarchy_graph(
            params.root_class.as_deref(),
            params.max_depth.unwrap_or(100),
            params.exclude_source_patterns.as_deref(),
        );

        match result {
            Ok(db_graph_data) => {
                // Convert database graph data to websocket graph data
                let ws_graph_data = ws_graph::GraphData {
                    nodes: db_graph_data.nodes.into_iter().map(|node| {
                        let id = node.id.clone();
                        ws_graph::Node {
                            id: node.id,
                            name: Some(id),
                            depth: Some(0),
                            color: match node.node_type {
                                arma3_database::models::class::NodeType::Normal => None,
                                arma3_database::models::class::NodeType::Removed => Some("#FF0000".to_string()),
                                arma3_database::models::class::NodeType::Orphaned => Some("#FFA500".to_string()),
                                arma3_database::models::class::NodeType::Affected => Some("#FFFF00".to_string()),
                            },
                            parent_id: node.parent_id,
                            container_class: node.container_class,
                            source_path: node.source_path,
                        }
                    }).collect(),
                    edges: db_graph_data.edges.into_iter().map(|edge| ws_graph::Edge {
                        source: edge.source,
                        target: edge.target,
                        color: None,
                    }).collect(),
                };
                
                let ws_response = WebSocketMessage {
                    action: "graph_data".to_string(),
                    data: Some(serde_json::to_value(ws_graph_data).unwrap()),
                };
                serde_json::to_string(&ws_response).unwrap()
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

    let _ = tx.send(response);
}

async fn handle_database_query(query: DatabaseQuery, tx: Arc<broadcast::Sender<String>>, db: Arc<DatabaseManager>) {
    let response = task::spawn_blocking(move || {
        match query.query_type {
            QueryType::GetClassHierarchy => {
                if let Some(params) = query.parameters {
                    if let Ok(request) = serde_json::from_value::<ClassHierarchyRequest>(params) {
                        let engine = GraphQueryEngine::new(&db);
                        match engine.build_class_hierarchy_graph(
                            request.root_class.as_deref(),
                            request.max_depth,
                            None,  // exclude_patterns
                        ) {
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