use crate::utils;
use crate::error::Result;
use arma3_tool_shared_models::GameDataClasses;
use prettytable::{Row, Cell};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Represents a report for class graph visualization
pub struct GraphReport {
    pub total_classes: usize,
    pub connected_classes: usize,
    pub isolated_classes: usize,
    pub total_edges: usize,
    pub nodes_file: String,
    pub edges_file: String,
}

/// Writer for generating graph visualization reports
pub struct GraphReportWriter<'a> {
    game_data: &'a GameDataClasses,
}

impl<'a> GraphReportWriter<'a> {
    /// Create a new graph report writer
    pub fn new(game_data: &'a GameDataClasses) -> Self {
        Self {
            game_data,
        }
    }

    /// Generate and write the graph report
    pub fn write_report(&self, output_path: &Path) -> Result<GraphReport> {
        // Create necessary directories
        utils::ensure_dir_exists(output_path)?;
        
        // Generate the graph data
        let (nodes, edges, stats, _) = self.generate_graph_data();
        
        // Write CSV graph files
        let nodes_file = output_path.join("nodes.csv");
        let edges_file = output_path.join("edges.csv");
        self.write_graph_files(&nodes, &edges, &nodes_file, &edges_file)?;
        
        // Create report text
        let report = GraphReport {
            total_classes: stats.total_classes,
            connected_classes: stats.connected_classes,
            isolated_classes: stats.isolated_classes,
            total_edges: stats.total_edges,
            nodes_file: nodes_file.display().to_string(),
            edges_file: edges_file.display().to_string(),
        };
        
        // Generate a summary report text
        self.write_graph_text_report(&report, output_path)?;
        
        Ok(report)
    }
    
    /// Generate graph data (nodes, edges, and statistics)
    fn generate_graph_data(&self) -> (Vec<Node>, Vec<Edge>, GraphStats, HashSet<usize>) {
        let class_count = self.game_data.classes.len();
        
        // Create all nodes
        let (nodes, node_ids) = {
            // Generate nodes
            let nodes: Vec<Node> = (0..class_count)
                .into_par_iter()
                .map(|i| {
                    let class = &self.game_data.classes[i];
                    
                    Node {
                        id: i,
                        name: class.name.clone(),
                    }
                })
                .collect();
            
            // Create node_ids map
            let mut node_ids = HashMap::with_capacity(class_count);
            for (i, class) in self.game_data.classes.iter().enumerate() {
                node_ids.insert(class.name.clone(), i);
            }
            
            (nodes, node_ids)
        };
        
        // Track which nodes have connections (for statistics only)
        let has_connections = Arc::new(Mutex::new(HashSet::<usize>::with_capacity(class_count)));
        
        // For collecting edges in parallel
        let edges_mutex = Arc::new(Mutex::new(Vec::new()));
        
        // Process classes in parallel for better performance
        let chunk_size = (class_count / rayon::current_num_threads()).max(100);
        self.game_data.classes.par_chunks(chunk_size).for_each(|chunk| {
            let mut local_edges = Vec::new();
            
            // Process each class in the chunk
            for class in chunk {
                if let Some(parent_name) = &class.parent {
                    if let Some(&parent_id) = node_ids.get(parent_name) {
                        let child_id = node_ids[&class.name];
                        
                        // Track that both nodes have connections (for statistics only)
                        {
                            let mut connections = has_connections.lock().unwrap();
                            connections.insert(parent_id);
                            connections.insert(child_id);
                        }
                        
                        // Add edge (collect locally first for better performance)
                        local_edges.push(Edge {
                            source: parent_id,
                            target: child_id,
                        });
                    }
                }
            }
            
            // Merge local results back to shared collection
            {
                let mut edges = edges_mutex.lock().unwrap();
                edges.extend(local_edges);
            }
        });
        
        // Get final list of edges and statistics
        let edges = Arc::try_unwrap(edges_mutex).unwrap().into_inner().unwrap();
        let has_connections = Arc::try_unwrap(has_connections).unwrap().into_inner().unwrap();
        
        // Count final statistics
        let total_edges = edges.len();
        let connected_classes = has_connections.len();
        let isolated_classes = class_count - connected_classes;
        
        // Return final data
        (nodes, edges, GraphStats {
            total_classes: class_count,
            connected_classes,
            isolated_classes,
            total_edges,
        }, has_connections)
    }
    
    /// Write separate CSV files for nodes and edges
    fn write_graph_files(&self, nodes: &[Node], edges: &[Edge], nodes_file_path: &Path, edges_file_path: &Path) -> Result<()> {
        // Get connected nodes set
        let connected_nodes = self.get_connected_nodes(edges);
        
        // Write nodes file
        {
            let file = File::create(nodes_file_path)?;
            let mut writer = BufWriter::with_capacity(1024 * 1024, file); // 1MB buffer
            
            // Write CSV header
            writeln!(writer, "id,label")?;
            
            // Write only connected nodes
            for node in nodes {
                if connected_nodes.contains(&node.id) {
                    writeln!(
                        writer, 
                        "{},{}",
                        node.id,
                        escape_csv(&node.name),
                    )?;
                }
            }
        }
        
        // Write edges file
        {
            let file = File::create(edges_file_path)?;
            let mut writer = BufWriter::with_capacity(1024 * 1024, file); // 1MB buffer
            
            // Write CSV header
            writeln!(writer, "source,target,label")?;
            
            // Write all edges
            for edge in edges {
                writeln!(
                    writer, 
                    "{},{},{}",
                    edge.source,
                    edge.target,
                    format!("{}->{}", 
                        escape_csv(&nodes[edge.source].name),
                        escape_csv(&nodes[edge.target].name)
                    )
                )?;
            }
        }
        
        Ok(())
    }
    
    /// Get a set of node IDs that have connections (either as source or target)
    fn get_connected_nodes(&self, edges: &[Edge]) -> HashSet<usize> {
        let mut connected = HashSet::new();
        
        for edge in edges {
            connected.insert(edge.source);
            connected.insert(edge.target);
        }
        
        connected
    }
    
    /// Generate a human-readable text report
    fn write_graph_text_report(&self, report: &GraphReport, output_path: &Path) -> Result<()> {
        let mut report_text = String::new();
        
        // Create report header
        report_text.push_str("=== CLASS GRAPH VISUALIZATION REPORT ===\n\n");
        
        // Create summary table
        let mut summary_table = utils::create_summary_table();
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Classes"),
            Cell::new(&report.total_classes.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Connected Classes"),
            Cell::new(&report.connected_classes.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Isolated Classes (Excluded)"),
            Cell::new(&report.isolated_classes.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Relationship Edges"),
            Cell::new(&report.total_edges.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Nodes CSV File"),
            Cell::new(&report.nodes_file),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Edges CSV File"),
            Cell::new(&report.edges_file),
        ]));
        
        report_text.push_str(&utils::table_to_string(&summary_table));
        report_text.push_str("\n\n");
        
        // Add usage instructions
        report_text.push_str("=== VISUALIZATION INSTRUCTIONS ===\n\n");
        report_text.push_str("The graph data has been exported as two separate CSV files:\n");
        report_text.push_str("1. nodes.csv - Contains only connected nodes with class names as labels\n");
        report_text.push_str("2. edges.csv - Contains parent-child relationships between classes\n");
        report_text.push_str("3. This format can be imported into various network visualization tools\n");
        report_text.push_str("4. For hierarchical visualizations, apply directed layouts\n");
        report_text.push_str("5. Isolated classes (with no parents or children) are excluded from the visualization\n");
        
        // Write to file
        utils::write_report(output_path, "graph_visualization_report.txt", &report_text)?;
        
        Ok(())
    }
}

/// Represents a node in the graph
#[derive(Clone, Debug)]
struct Node {
    id: usize,
    name: String,
}

/// Represents an edge in the graph
#[derive(Clone, Debug)]
struct Edge {
    source: usize,
    target: usize,
}

/// Statistics about the graph
struct GraphStats {
    total_classes: usize,
    connected_classes: usize,
    isolated_classes: usize,
    total_edges: usize,
}

/// Escape a string for CSV output
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        // Quote the string and escape any quotes within it
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
} 