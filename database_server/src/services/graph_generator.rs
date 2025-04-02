use rayon::prelude::*;
use std::sync::Arc;
use parking_lot::Mutex;
use rand::Rng;

use crate::models::graph::{Node, Edge, GraphData, GraphConfig};

pub fn generate_graph(config: GraphConfig) -> GraphData {
    
    println!("Generating tree with {} nodes", config.node_count);
    
    // Pre-allocate vectors with known capacity
    let mut nodes = Vec::with_capacity(config.node_count);
    let edges = Vec::with_capacity(config.node_count - 1); // Tree has n-1 edges
    
    // Calculate max depth based on expected tree depth (log2 of node count is a good approximation)
    let max_depth = (config.node_count as f64).log2() as usize * 2;
    println!("Estimated max depth: {}", max_depth);
    
    // Create root node at the center
    nodes.push(Node {
        id: "0".to_string(),
        name: Some("Root".to_string()),
        depth: Some(0),
        color: None,
    });
    
    // Generate tree in parallel chunks
    let node_count = config.node_count.min(100000); // Cap node count to avoid excessive memory usage
    let chunk_size = 1000;
    let nodes_arc = Arc::new(Mutex::new(nodes));
    let edges_arc = Arc::new(Mutex::new(edges));
    
    // Process in parallel chunks
    (1..node_count).collect::<Vec<_>>()
        .par_chunks(chunk_size)
        .for_each(|chunk| {
            let mut local_nodes = Vec::with_capacity(chunk.len());
            let mut local_edges = Vec::with_capacity(chunk.len());
            let mut local_rng = rand::rng();
            
            for &i in chunk {
                // Choose a random parent from existing nodes (0 to i-1)
                let parent_idx = local_rng.random_range(0..i);
                
                // Add node
                let node = Node {
                    id: i.to_string(),
                    name: Some(format!("Node {}", i)),
                    depth: Some(1),
                    color: None,
                };
                
                // Add edge from parent to this node
                let edge = Edge {
                    source: parent_idx.to_string(),
                    target: i.to_string(),
                    color: None,
                };
                
                local_nodes.push(node);
                local_edges.push(edge);
            }
            
            // Merge local results
            let mut nodes = nodes_arc.lock();
            let mut edges = edges_arc.lock();
            nodes.extend(local_nodes);
            edges.extend(local_edges);
        });
    
    // Safely unwrap the Arc<Mutex> into the inner values
    let nodes = Arc::try_unwrap(nodes_arc)
        .expect("Failed to unwrap nodes Arc")
        .into_inner();
    let edges = Arc::try_unwrap(edges_arc)
        .expect("Failed to unwrap edges Arc")
        .into_inner();
    
    GraphData { nodes, edges }
} 