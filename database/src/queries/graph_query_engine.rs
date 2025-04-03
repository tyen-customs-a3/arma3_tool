use crate::DatabaseManager;
use crate::error::Result;
use crate::models::class::{ClassModel, GraphNode, GraphEdge, NodeType};
use crate::queries::class_repository::ClassRepository;
use petgraph::{Graph, Directed};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use log::debug;
use rusqlite::OptionalExtension;
use serde::Serialize;
use rayon::prelude::*;

/// Engine for graph-related database operations
pub struct GraphQueryEngine<'a> {
    db: &'a DatabaseManager,
}

impl<'a> GraphQueryEngine<'a> {
    /// Create a new graph query engine
    pub fn new(db: &'a DatabaseManager) -> Self {
        Self { db }
    }
    
    /// Build a class hierarchy graph starting from root class or all roots
    pub fn build_class_hierarchy_graph(
        &self,
        root_class: Option<&str>,
        max_depth: i32,
        exclude_patterns: Option<&[String]>,
    ) -> Result<GraphData> {
        // Create repositories
        let class_repo = ClassRepository::new(self.db);
        
        // Get hierarchy nodes from database
        let hierarchy_nodes = if let Some(root) = root_class {
            class_repo.get_hierarchy(root, max_depth)?
        } else {
            class_repo.get_full_hierarchy(max_depth)?
        };

        debug!("Found {} total classes in hierarchy", hierarchy_nodes.len());
        
        // Print the exclude patterns
        debug!("Exclude patterns: {:?}", exclude_patterns);

        // Pre-process exclude patterns if any exist
        let exclude_patterns = if let Some(patterns) = exclude_patterns {
            Some(Arc::new(patterns.iter()
                .map(|p| p.to_lowercase())
                .collect::<Vec<_>>()))
        } else {
            None
        };

        // Batch fetch all source paths upfront
        let source_paths = if exclude_patterns.is_some() {
            let source_indices: Vec<_> = hierarchy_nodes.iter()
                .filter_map(|node| node.source_file_index)
                .collect();
            
            if !source_indices.is_empty() {
                let mut path_map = HashMap::new();
                for idx in source_indices {
                    if let Ok(Some(path)) = class_repo.get_source_path(idx) {
                        path_map.insert(idx, path.to_lowercase());
                    }
                }
                Some(Arc::new(path_map))
            } else {
                None
            }
        } else {
            None
        };

        // Process nodes in parallel
        let filtered_nodes: Vec<_> = hierarchy_nodes.par_iter()
            .filter_map(|node| {
                // Check if we should include this node based on source patterns
                let should_include = if let Some(patterns) = &exclude_patterns {
                    let node_id_lower = node.id.to_lowercase();
                    
                    // Check if node ID matches any pattern
                    let matches_id = patterns.par_iter()
                        .any(|pattern| node_id_lower.contains(pattern));
                    
                    // Check if source path matches any pattern
                    let matches_source = if let Some(source_idx) = node.source_file_index {
                        if let Some(paths) = &source_paths {
                            if let Some(path) = paths.get(&source_idx) {
                                patterns.par_iter().any(|pattern| path.contains(pattern))
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    !(matches_id || matches_source)
                } else {
                    true
                };

                if should_include {
                    // Get source path if available
                    let source_path = if let Some(source_idx) = node.source_file_index {
                        class_repo.get_source_path(source_idx).ok().flatten()
                    } else {
                        None
                    };

                    Some((node, source_path))
                } else {
                    None
                }
            })
            .collect();

        // Build nodes and edges from filtered results
        let mut nodes = Vec::with_capacity(filtered_nodes.len());
        let mut edges = Vec::with_capacity(filtered_nodes.len());

        for (node, source_path) in filtered_nodes {
            nodes.push(GraphNode {
                id: node.id.clone(),
                node_type: NodeType::Normal,
                source_file_index: node.source_file_index,
                parent_id: node.parent_id.clone(),
                container_class: node.container_class.clone(),
                source_path,
            });
            
            // Create edge if there's a parent
            if let Some(parent_id) = &node.parent_id {
                edges.push(GraphEdge {
                    source: parent_id.clone(),
                    target: node.id.clone(),
                    weight: 1.0,
                });
            }
        }

        let filtered_count = hierarchy_nodes.len() - nodes.len();
        if let Some(exclude) = &exclude_patterns {
            debug!("Excluded {} out of {} classes using patterns: {:?}", 
                filtered_count, 
                hierarchy_nodes.len(),
                exclude
            );
        }
        
        Ok(GraphData { nodes, edges })
    }
    
    /// Build a PBO dependency graph
    pub fn build_pbo_dependency_graph(&self) -> Result<GraphData> {
        self.db.with_connection(|conn| {
            // Use the file_index_mapping table to map indices to PBO IDs
            let query = "
                WITH source_indices AS (
                    SELECT c.id AS class_id, c.parent_id, c.source_file_index
                    FROM classes c
                    WHERE c.source_file_index IS NOT NULL
                ),
                index_to_pbo AS (
                    SELECT file_index, 
                           COALESCE(pbo_id, normalized_path) AS pbo_id
                    FROM file_index_mapping
                )
                SELECT DISTINCT 
                    parent_idx.pbo_id AS parent_pbo_id, 
                    child_idx.pbo_id AS child_pbo_id,
                    COUNT(*) AS dependency_count
                FROM source_indices child
                JOIN source_indices parent ON child.parent_id = parent.class_id
                JOIN index_to_pbo parent_idx ON parent.source_file_index = parent_idx.file_index
                JOIN index_to_pbo child_idx ON child.source_file_index = child_idx.file_index
                WHERE 
                    child.source_file_index != parent.source_file_index
                    AND parent_idx.pbo_id != child_idx.pbo_id
                GROUP BY parent_pbo_id, child_pbo_id
                ORDER BY dependency_count DESC
            ";
            
            let mut stmt = conn.prepare(query)?;
            let rows = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?, // parent_pbo_id
                    row.get::<_, String>(1)?, // child_pbo_id
                    row.get::<_, i64>(2)?,    // dependency_count
                ))
            })?;
            
            // Process rows to collect nodes and edges
            let mut nodes_set = HashSet::new();
            let mut edges = Vec::new();
            
            for row_result in rows {
                let (parent_pbo_id, child_pbo_id, count) = row_result?;
                
                nodes_set.insert(parent_pbo_id.clone());
                nodes_set.insert(child_pbo_id.clone());
                
                // Add edge
                edges.push(GraphEdge {
                    source: parent_pbo_id,
                    target: child_pbo_id,
                    weight: count as f32,
                });
            }
            
            // Convert nodes set to vector
            let nodes = nodes_set.into_iter()
                .map(|pbo_id| {
                    // Try to get source_file_index for this PBO ID
                    let source_file_index = conn.query_row(
                        "SELECT file_index FROM file_index_mapping 
                         WHERE COALESCE(pbo_id, normalized_path) = ?1 LIMIT 1",
                        [&pbo_id],
                        |row| row.get::<_, i64>(0)
                    ).optional().ok().flatten().map(|idx| idx as usize);
                    
                    GraphNode {
                        id: pbo_id,
                        node_type: NodeType::Normal,
                        source_file_index,
                        parent_id: None,
                        container_class: None,
                        source_path: None,
                    }
                })
                .collect();
            
            Ok(GraphData { nodes, edges })
        })
    }
    
    /// Run impact analysis for class removal
    pub fn impact_analysis(&self, classes_to_remove: &[String]) -> Result<ImpactAnalysisResult> {
        debug!("Running impact analysis for classes: {:?}", classes_to_remove);
        
        if classes_to_remove.is_empty() {
            return Ok(ImpactAnalysisResult {
                removed_classes: Vec::new(),
                orphaned_classes: Vec::new(),
                affected_classes: Vec::new(),
                graph_data: GraphData::default(),
            });
        }
        
        // Create repositories
        let class_repo = ClassRepository::new(self.db);
        
        // Find orphaned classes (direct children of classes to be removed)
        let orphaned_models = class_repo.find_orphaned_by_parent_removal(classes_to_remove)?;
        let orphaned_ids: Vec<String> = orphaned_models.iter()
            .map(|c| c.id.clone())
            .collect();
        
        debug!("Found orphaned classes: {:?}", orphaned_ids);
        
        // Find affected classes (children of orphaned classes)
        let affected_models = class_repo.find_affected_children(&orphaned_ids, 10)?;
        let affected_ids: Vec<String> = affected_models.iter()
            .map(|c| c.id.clone())
            .collect();
            
        debug!("Found affected classes: {:?}", affected_ids);
        
        // Build graph data
        let mut graph_data = GraphData::default();
        
        // Add removed classes as nodes
        for class_id in classes_to_remove {
            if let Ok(Some(class)) = class_repo.get(class_id) {
                // Add the class node
                graph_data.nodes.push(GraphNode {
                    id: class.id.clone(),
                    node_type: NodeType::Removed,
                    source_file_index: class.source_file_index,
                    parent_id: class.parent_id.clone(),
                    container_class: class.container_class.clone(),
                    source_path: class.source_file_index.and_then(|idx| class_repo.get_source_path(idx).ok().flatten()),
                });
                
                // Add edge from parent if it exists and not in remove list
                if let Some(parent_id) = &class.parent_id {
                    if !classes_to_remove.contains(parent_id) {
                        // Add parent node if not already in the graph
                        if !graph_data.nodes.iter().any(|n| &n.id == parent_id) {
                            if let Ok(Some(parent)) = class_repo.get(parent_id) {
                                graph_data.nodes.push(GraphNode {
                                    id: parent.id.clone(),
                                    node_type: NodeType::Normal,
                                    source_file_index: parent.source_file_index,
                                    parent_id: parent.parent_id.clone(),
                                    container_class: parent.container_class.clone(),
                                    source_path: parent.source_file_index.and_then(|idx| class_repo.get_source_path(idx).ok().flatten()),
                                });
                            }
                        }
                        
                        // Add edge
                        graph_data.edges.push(GraphEdge {
                            source: parent_id.clone(),
                            target: class.id.clone(),
                            weight: 1.0,
                        });
                    }
                }
            }
        }
        
        // Add orphaned classes
        for class in &orphaned_models {
            // Add node
            graph_data.nodes.push(GraphNode {
                id: class.id.clone(),
                node_type: NodeType::Orphaned,
                source_file_index: class.source_file_index,
                parent_id: class.parent_id.clone(),
                container_class: class.container_class.clone(),
                source_path: class.source_file_index.and_then(|idx| class_repo.get_source_path(idx).ok().flatten()),
            });
            
            // Add edge from parent (which is in the remove list)
            if let Some(parent_id) = &class.parent_id {
                graph_data.edges.push(GraphEdge {
                    source: parent_id.clone(),
                    target: class.id.clone(),
                    weight: 1.0,
                });
            }
        }
        
        // Add affected classes
        for class in &affected_models {
            // Add node
            graph_data.nodes.push(GraphNode {
                id: class.id.clone(),
                node_type: NodeType::Affected,
                source_file_index: class.source_file_index,
                parent_id: class.parent_id.clone(),
                container_class: class.container_class.clone(),
                source_path: class.source_file_index.and_then(|idx| class_repo.get_source_path(idx).ok().flatten()),
            });
            
            // Add edge from parent
            if let Some(parent_id) = &class.parent_id {
                // Add parent node if not in our lists already
                let parent_in_graph = graph_data.nodes.iter().any(|n| &n.id == parent_id);
                
                if !parent_in_graph {
                    if let Ok(Some(parent)) = class_repo.get(parent_id) {
                        let node_type = if orphaned_ids.contains(parent_id) {
                            NodeType::Orphaned
                        } else if affected_ids.contains(parent_id) {
                            NodeType::Affected
                        } else {
                            NodeType::Normal
                        };
                        
                        graph_data.nodes.push(GraphNode {
                            id: parent.id.clone(),
                            node_type: node_type,
                            source_file_index: parent.source_file_index,
                            parent_id: parent.parent_id.clone(),
                            container_class: parent.container_class.clone(),
                            source_path: parent.source_file_index.and_then(|idx| class_repo.get_source_path(idx).ok().flatten()),
                        });
                    }
                }
                
                // Add edge
                graph_data.edges.push(GraphEdge {
                    source: parent_id.clone(),
                    target: class.id.clone(),
                    weight: 1.0,
                });
            }
        }
        
        // Return impact analysis result
        Ok(ImpactAnalysisResult {
            removed_classes: classes_to_remove.to_vec(),
            orphaned_classes: orphaned_ids,
            affected_classes: affected_ids,
            graph_data,
        })
    }
    
    /// Build a Petgraph from class models
    pub fn build_petgraph(&self, classes: &[ClassModel]) -> Graph<String, f32, Directed> {
        let mut graph = Graph::new();
        let mut node_map = HashMap::new();
        
        // Add all nodes first
        for class in classes {
            let idx = graph.add_node(class.id.clone());
            node_map.insert(class.id.clone(), idx);
        }
        
        // Add edges
        for class in classes {
            if let Some(parent_id) = &class.parent_id {
                if let Some(&parent_idx) = node_map.get(parent_id) {
                    if let Some(&child_idx) = node_map.get(&class.id) {
                        graph.add_edge(parent_idx, child_idx, 1.0);
                    }
                }
            }
        }
        
        graph
    }
}

/// Graph data structure for visualization
#[derive(Debug, Clone, Default, Serialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Impact analysis result
#[derive(Debug, Clone, Serialize)]
pub struct ImpactAnalysisResult {
    pub removed_classes: Vec<String>,
    pub orphaned_classes: Vec<String>,
    pub affected_classes: Vec<String>,
    pub graph_data: GraphData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseManager;
    use tempfile::tempdir;
    
    #[test]
    fn test_graph_query_engine() {
        // Create a temporary database
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseManager::new(&db_path).unwrap();
        
        // Create the graph query engine
        let engine = GraphQueryEngine::new(&db);
        
        // Create some test classes
        let class1 = ClassModel::new(
            "Class1".to_string(), 
            None::<String>,
            None::<String>,
            Some(1)
        );
        let class2 = ClassModel::new(
            "Class2".to_string(), 
            Some("Class1".to_string()),
            None::<String>,
            Some(2)
        );
        let class3 = ClassModel::new(
            "Class3".to_string(), 
            Some("Class2".to_string()),
            None::<String>,
            Some(3)
        );
        let class4 = ClassModel::new(
            "Class4".to_string(), 
            Some("Class1".to_string()),
            None::<String>,
            Some(4)
        );
        
        // Create a petgraph
        let graph = engine.build_petgraph(&[class1, class2, class3, class4]);
        
        // Verify graph structure
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);
    }
} 