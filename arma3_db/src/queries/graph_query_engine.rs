use crate::DatabaseManager;
use crate::error::Result;
use crate::models::class::{ClassModel, GraphNode, GraphEdge, NodeType};
use crate::queries::class_repository::ClassRepository;
use petgraph::{Graph, Directed};
use std::collections::{HashMap, HashSet};
use log::debug;

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
    ) -> Result<GraphData> {
        // Create repositories
        let class_repo = ClassRepository::new(self.db);
        
        // Get hierarchy nodes from database
        let hierarchy_nodes = if let Some(root) = root_class {
            class_repo.get_hierarchy(root, max_depth)?
        } else {
            class_repo.get_full_hierarchy(max_depth)?
        };
        
        // Convert to graph data
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        
        // Create nodes
        for node in &hierarchy_nodes {
            nodes.push(GraphNode {
                id: node.id.clone(),
                node_type: NodeType::Normal,
                source_pbo_id: node.source_pbo_id.clone(),
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
        
        Ok(GraphData { nodes, edges })
    }
    
    /// Build a PBO dependency graph
    pub fn build_pbo_dependency_graph(&self) -> Result<GraphData> {
        self.db.with_connection(|conn| {
            // Query to get PBO dependencies (where classes in one PBO extend classes in another)
            let query = "
                WITH class_pbo_mapping AS (
                    SELECT c.id AS class_id, c.parent_id, c.source_pbo_id
                    FROM classes c
                    WHERE c.source_pbo_id IS NOT NULL
                )
                SELECT DISTINCT 
                    parent.source_pbo_id AS parent_pbo, 
                    child.source_pbo_id AS child_pbo,
                    COUNT(*) AS dependency_count
                FROM class_pbo_mapping child
                JOIN class_pbo_mapping parent ON child.parent_id = parent.class_id
                WHERE 
                    child.source_pbo_id != parent.source_pbo_id
                GROUP BY parent_pbo, child_pbo
                ORDER BY dependency_count DESC
            ";
            
            let mut stmt = conn.prepare(query)?;
            let rows = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?, // parent_pbo
                    row.get::<_, String>(1)?, // child_pbo
                    row.get::<_, i64>(2)?,    // dependency_count
                ))
            })?;
            
            // Track unique PBOs
            let mut pbo_set = HashSet::new();
            let mut edges = Vec::new();
            
            // Process rows
            for row_result in rows {
                let (parent_pbo, child_pbo, count) = row_result?;
                
                pbo_set.insert(parent_pbo.clone());
                pbo_set.insert(child_pbo.clone());
                
                // Add edge with weight based on dependency count
                edges.push(GraphEdge {
                    source: parent_pbo,
                    target: child_pbo,
                    weight: count as f32,
                });
            }
            
            // Create nodes for all unique PBOs
            let nodes = pbo_set.into_iter()
                .map(|id| GraphNode {
                    id,
                    node_type: NodeType::Normal,
                    source_pbo_id: None,
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
                    source_pbo_id: class.source_pbo_id.clone(),
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
                                    source_pbo_id: parent.source_pbo_id.clone(),
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
                source_pbo_id: class.source_pbo_id.clone(),
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
                source_pbo_id: class.source_pbo_id.clone(),
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
                            node_type,
                            source_pbo_id: parent.source_pbo_id.clone(),
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
        
        Ok(ImpactAnalysisResult {
            removed_classes: classes_to_remove.to_vec(),
            orphaned_classes: orphaned_ids,
            affected_classes: affected_ids,
            graph_data,
        })
    }
    
    /// Convert database class models to a Petgraph representation
    pub fn build_petgraph(&self, classes: &[ClassModel]) -> Graph<String, f32, Directed> {
        let mut graph = Graph::<String, f32, Directed>::new();
        let mut node_map = HashMap::new();
        
        // First add all nodes
        for class in classes {
            let node_idx = graph.add_node(class.id.clone());
            node_map.insert(class.id.clone(), node_idx);
        }
        
        // Then add edges
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

/// Graph data for visualization
#[derive(Debug, Default, Clone)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone)]
pub struct ImpactAnalysisResult {
    pub removed_classes: Vec<String>,
    pub orphaned_classes: Vec<String>,
    pub affected_classes: Vec<String>,
    pub graph_data: GraphData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::models::class::ClassModel;
    use env_logger;
    use log::info;
    
    #[test]
    fn test_graph_query_engine() {
        // Initialize logger for tests
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
            
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let class_repo = ClassRepository::new(&db);
        let graph_engine = GraphQueryEngine::new(&db);
        
        // Create test classes without PBO references to avoid foreign key constraints
        let class1 = ClassModel::new("Class1".to_string(), None::<String>, None::<String>, Some(1));
        let class2 = ClassModel::new("Class2".to_string(), Some("Class1".to_string()), None::<String>, Some(2));
        let class3 = ClassModel::new("Class3".to_string(), Some("Class2".to_string()), None::<String>, Some(3));
        let class4 = ClassModel::new("Class4".to_string(), Some("Class1".to_string()), None::<String>, Some(4));
        
        info!("Creating test classes: Class1, Class2 (parent: Class1), Class3 (parent: Class2), Class4 (parent: Class1)");
        
        // Insert classes
        class_repo.create(&class1).unwrap();
        class_repo.create(&class2).unwrap();
        class_repo.create(&class3).unwrap();
        class_repo.create(&class4).unwrap();
        
        // Test hierarchy graph
        let graph_data = graph_engine.build_class_hierarchy_graph(Some("Class1"), 2).unwrap();
        
        debug!("Hierarchy graph nodes: {:?}", graph_data.nodes);
        debug!("Hierarchy graph edges: {:?}", graph_data.edges);
        
        assert_eq!(graph_data.nodes.len(), 4); // Class1, Class2, Class3, Class4 (Class3 is included because max_depth is 2)
        assert_eq!(graph_data.edges.len(), 3); // Class1->Class2, Class1->Class4, Class2->Class3
        
        // Test impact analysis
        let impact = graph_engine.impact_analysis(&["Class1".to_string()]).unwrap();
        
        debug!("Impact analysis removed classes: {:?}", impact.removed_classes);
        debug!("Impact analysis orphaned classes: {:?}", impact.orphaned_classes);
        debug!("Impact analysis affected classes: {:?}", impact.affected_classes);
        
        assert_eq!(impact.removed_classes.len(), 1);
        assert_eq!(impact.orphaned_classes.len(), 2); // Class2 and Class4
        assert_eq!(impact.affected_classes.len(), 1); // Class3 only
    }
} 