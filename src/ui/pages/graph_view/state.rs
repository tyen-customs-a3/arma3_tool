use egui_graphs::Graph;
use petgraph::{Graph as PetGraph, Directed, stable_graph::{StableGraph, NodeIndex}};
use arma3_tool_shared_models::{GameDataClass, GameDataClasses};
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use rfd::FileDialog;

#[derive(PartialEq, Clone)]
pub enum ViewMode {
    ClassHierarchy,
    PboAnalysis,
    Custom,
    ImpactAnalysis,
}

#[derive(PartialEq, Clone)]
pub enum HierarchyMode {
    FromRoot,
    FromClass,
}

#[derive(Clone, PartialEq)]
pub enum NodeStatus {
    Normal,
    Removed,
    Orphaned,
    Affected,
}

#[derive(Default)]
pub struct AnalysisMetrics {
    pub size_on_disk: u64,
    pub num_classes: usize,
    pub num_dependencies: usize,
    pub analysis_time_ms: u64,
}

// Structure to hold class dependency information for faster lookups
pub struct ClassDependencyCache {
    pub children: HashMap<String, Vec<String>>,
    pub parents: HashMap<String, Option<String>>,
    pub class_to_pbo: HashMap<String, PathBuf>,
    pub is_dirty: bool,
}

impl Default for ClassDependencyCache {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            parents: HashMap::new(),
            class_to_pbo: HashMap::new(),
            is_dirty: true,
        }
    }
}

pub struct GraphViewState {
    // Graph state
    pub graph: Option<Graph<String, ()>>,
    pub analysis_graph: Option<PetGraph<String, f32, Directed>>,
    // View settings
    pub max_depth: usize,
    pub root_class: String,
    pub view_mode: ViewMode,
    pub hierarchy_mode: HierarchyMode,
    // Filter settings
    pub class_filter: String,
    pub pbo_filter: String,
    // Filtered lists for selection
    pub filtered_classes: Vec<String>,
    pub filtered_pbos: Vec<PathBuf>,
    // Analysis data
    pub metrics: AnalysisMetrics,
    // Cache
    pub dependency_cache: ClassDependencyCache,
    // Impact analysis
    pub classes_to_remove: HashSet<String>,
    pub orphaned_classes: HashSet<String>,
    pub affected_classes: HashSet<String>,
    pub impact_file_path: Option<PathBuf>,
    // Selected node
    pub selected_node: Option<String>,
}

impl Default for GraphViewState {
    fn default() -> Self {
        Self {
            graph: None,
            analysis_graph: None,
            max_depth: 3,
            root_class: String::new(),
            view_mode: ViewMode::ClassHierarchy,
            hierarchy_mode: HierarchyMode::FromRoot,
            class_filter: String::new(),
            pbo_filter: String::new(),
            filtered_classes: Vec::new(),
            filtered_pbos: Vec::new(),
            metrics: AnalysisMetrics::default(),
            dependency_cache: ClassDependencyCache::default(),
            classes_to_remove: HashSet::new(),
            orphaned_classes: HashSet::new(),
            affected_classes: HashSet::new(),
            impact_file_path: None,
            selected_node: None,
        }
    }
}

impl GraphViewState {
    // Update the dependency cache if needed
    pub fn ensure_dependency_cache(&mut self, game_data: &GameDataClasses) {
        if self.dependency_cache.is_dirty {
            let start = Instant::now();
            
            // Clear existing cache
            self.dependency_cache.children.clear();
            self.dependency_cache.parents.clear();
            self.dependency_cache.class_to_pbo.clear();
            
            // Build parents mapping
            for class in &game_data.classes {
                self.dependency_cache.parents.insert(class.name.clone(), class.parent.clone());
                
                // Map class to its PBO
                if let Some(idx) = class.source_file_index {
                    if let Some(source_file) = game_data.file_sources.get(idx) {
                        self.dependency_cache.class_to_pbo.insert(class.name.clone(), source_file.clone());
                    }
                }
            }
            
            // Build children mapping
            for class in &game_data.classes {
                if let Some(parent) = &class.parent {
                    self.dependency_cache.children
                        .entry(parent.clone())
                        .or_insert_with(Vec::new)
                        .push(class.name.clone());
                }
            }
            
            // Sort children for consistent results
            for children in self.dependency_cache.children.values_mut() {
                children.sort();
            }
            
            self.dependency_cache.is_dirty = false;
            
            // Update metrics
            self.metrics.analysis_time_ms = start.elapsed().as_millis() as u64;
        }
    }

    pub fn update_filtered_classes(&mut self, game_data: &GameDataClasses) {
        let start = Instant::now();
        self.filtered_classes.clear();
        let filter = self.class_filter.to_lowercase();
        
        for class in &game_data.classes {
            if class.name.to_lowercase().contains(&filter) {
                self.filtered_classes.push(class.name.clone());
            }
        }
        
        self.filtered_classes.sort();
        self.metrics.analysis_time_ms = start.elapsed().as_millis() as u64;
    }

    pub fn update_filtered_pbos(&mut self, game_data: &GameDataClasses) {
        let start = Instant::now();
        self.filtered_pbos.clear();
        let filter = self.pbo_filter.to_lowercase();
        
        let mut unique_pbos = HashSet::new();
        
        // Collect unique PBO paths from classes
        for class in &game_data.classes {
            if let Some(idx) = class.source_file_index {
                if let Some(source_file) = game_data.file_sources.get(idx) {
                    if filter.is_empty() || source_file.to_string_lossy().to_lowercase().contains(&filter) {
                        unique_pbos.insert(source_file.clone());
                    }
                }
            }
        }
        
        self.filtered_pbos = unique_pbos.into_iter().collect();
        self.filtered_pbos.sort();
        self.metrics.analysis_time_ms = start.elapsed().as_millis() as u64;
    }

    pub fn update_graph(&mut self, game_data: &GameDataClasses) {
        // Ensure the dependency cache is up to date
        self.ensure_dependency_cache(game_data);
        
        let start = Instant::now();
        
        match self.view_mode {
            ViewMode::ClassHierarchy => self.update_hierarchy_graph(game_data),
            ViewMode::PboAnalysis => self.update_pbo_graph(game_data),
            ViewMode::Custom => self.update_custom_graph(game_data),
            ViewMode::ImpactAnalysis => self.update_impact_analysis_graph(game_data),
        }
        
        self.metrics.analysis_time_ms = start.elapsed().as_millis() as u64;
    }

    fn update_hierarchy_graph(&mut self, game_data: &GameDataClasses) {
        // Create a new stable graph
        let stable_graph = StableGraph::default();
        let mut graph = Graph::from(&stable_graph);
        
        // Track nodes we've already added to avoid duplicates
        let mut added_nodes = HashMap::new();
        
        match self.hierarchy_mode {
            HierarchyMode::FromRoot => {
                // Get root classes (those without parents)
                let root_classes: Vec<&str> = self.dependency_cache.parents.iter()
                    .filter(|(_, parent)| parent.is_none())
                    .map(|(class, _)| class.as_str())
                    .collect();

                for root_class in root_classes {
                    if let Some(class) = game_data.classes.iter().find(|c| c.name == root_class) {
                        self.build_hierarchy(&mut graph, &mut added_nodes, class, game_data, 0);
                    }
                }
            }
            HierarchyMode::FromClass => {
                if let Some(start_class) = game_data.classes.iter()
                    .find(|c| c.name == self.root_class)
                {
                    self.build_hierarchy(&mut graph, &mut added_nodes, start_class, game_data, 0);
                }
            }
        }

        self.graph = Some(graph);
    }
    
    fn build_hierarchy(
        &self,
        graph: &mut Graph<String, ()>,
        added_nodes: &mut HashMap<String, NodeIndex>,
        class: &GameDataClass,
        game_data: &GameDataClasses,
        depth: usize,
    ) {
        if depth >= self.max_depth {
            return;
        }

        // Add current node or get existing node ID
        let class_node = if let Some(&node_id) = added_nodes.get(&class.name) {
            node_id
        } else {
            let node_id = graph.add_node(class.name.clone());
            added_nodes.insert(class.name.clone(), node_id);
            node_id
        };

        // Find all direct child classes using cache
        if let Some(children) = self.dependency_cache.children.get(&class.name) {
            for child_name in children {
                if let Some(child_class) = game_data.classes.iter().find(|c| &c.name == child_name) {
                    // Add child node or get existing node ID
                    let child_node = if let Some(&node_id) = added_nodes.get(&child_class.name) {
                        node_id
                    } else {
                        let node_id = graph.add_node(child_class.name.clone());
                        added_nodes.insert(child_class.name.clone(), node_id);
                        node_id
                    };
                    
                    // Add edge from parent to child
                    graph.add_edge(class_node, child_node, ());
                    
                    // Recursively add child's children
                    self.build_hierarchy(graph, added_nodes, child_class, game_data, depth + 1);
                }
            }
        }
    }

    fn update_pbo_graph(&mut self, game_data: &GameDataClasses) {
        let stable_graph = StableGraph::default();
        let mut graph = Graph::from(&stable_graph);
        let mut metrics = AnalysisMetrics::default();
        
        // Create nodes for each PBO and track metrics
        let mut pbo_nodes = HashMap::new();
        let mut pbo_sizes = HashMap::new();
        let mut pbo_class_counts = HashMap::new();
        
        // First pass - collect PBO data
        for (class_name, pbo_path) in &self.dependency_cache.class_to_pbo {
            let pbo_name = pbo_path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            
            // Increment class count for this PBO
            *pbo_class_counts.entry(pbo_name.clone()).or_insert(0) += 1;
            
            // Get PBO size if we haven't already
            if !pbo_sizes.contains_key(&pbo_name) {
                if let Ok(metadata) = std::fs::metadata(pbo_path) {
                    pbo_sizes.insert(pbo_name.clone(), metadata.len());
                    metrics.size_on_disk += metadata.len();
                }
            }
        }
        
        // Second pass - create nodes
        for (pbo_name, class_count) in &pbo_class_counts {
            metrics.num_classes += *class_count;
            let node = graph.add_node(format!("{} ({} classes)", pbo_name, class_count));
            pbo_nodes.insert(pbo_name.clone(), node);
        }
        
        // Add edges based on class dependencies 
        for (class_name, parent) in &self.dependency_cache.parents {
            if let Some(parent_name) = parent {
                // Get PBOs for this class and its parent
                if let (Some(class_pbo), Some(parent_pbo)) = (
                    self.dependency_cache.class_to_pbo.get(class_name).map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string()),
                    self.dependency_cache.class_to_pbo.get(parent_name).map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
                ) {
                    // Only add edge if the PBOs are different
                    if class_pbo != parent_pbo {
                        if let (Some(&from), Some(&to)) = (pbo_nodes.get(&parent_pbo), pbo_nodes.get(&class_pbo)) {
                            // Check if edge already exists - use graph API directly
                            // This is a simplification - we won't check for existing edges
                            // since in egui_graphs we can't easily access the raw graph
                            graph.add_edge(from, to, ());
                            metrics.num_dependencies += 1;
                        }
                    }
                }
            }
        }
        
        self.graph = Some(graph);
        self.metrics = metrics;
    }

    fn update_custom_graph(&mut self, game_data: &GameDataClasses) {
        // Create a more detailed analysis graph using petgraph
        let mut graph = PetGraph::<String, f32, Directed>::new();
        
        // Add nodes for all classes
        let mut class_nodes = HashMap::new();
        for class in &game_data.classes {
            let node_idx = graph.add_node(class.name.clone());
            class_nodes.insert(&class.name, node_idx);
        }
        
        // Add edges based on cached dependencies
        for (class_name, parent_opt) in &self.dependency_cache.parents {
            if let Some(parent) = parent_opt {
                if let (Some(&from), Some(&to)) = (class_nodes.get(parent), class_nodes.get(class_name)) {
                    // TODO: Calculate better edge weights based on metrics
                    let weight = 1.0;
                    graph.add_edge(from, to, weight);
                }
            }
        }
        
        self.analysis_graph = Some(graph);
    }
    
    pub fn load_classes_to_remove(&mut self, file_path: PathBuf) -> Result<(), String> {
        match std::fs::read_to_string(&file_path) {
            Ok(content) => {
                self.classes_to_remove.clear();
                
                for line in content.lines() {
                    let class = line.trim();
                    if !class.is_empty() && !class.starts_with('#') {
                        self.classes_to_remove.insert(class.to_string());
                    }
                }
                
                self.impact_file_path = Some(file_path);
                Ok(())
            },
            Err(e) => Err(format!("Failed to read file: {}", e)),
        }
    }
    
    fn update_impact_analysis_graph(&mut self, game_data: &GameDataClasses) {
        // Create a new stable graph
        let stable_graph = StableGraph::default();
        let mut graph = Graph::from(&stable_graph);
        
        // Track nodes we've already added to avoid duplicates
        let mut added_nodes = HashMap::new();
        let mut node_status = HashMap::new();
        
        // Initialize status of removed classes
        for class in &self.classes_to_remove {
            node_status.insert(class.clone(), NodeStatus::Removed);
        }
        
        // First find all orphaned classes (classes whose parent will be removed)
        self.orphaned_classes.clear();
        self.affected_classes.clear();
        
        // First collect all orphaned classes
        let mut newly_orphaned = Vec::new();
        for (class_name, parent_opt) in &self.dependency_cache.parents {
            if let Some(parent) = parent_opt {
                if self.classes_to_remove.contains(parent) {
                    // This class is orphaned by removal
                    self.orphaned_classes.insert(class_name.clone());
                    node_status.insert(class_name.clone(), NodeStatus::Orphaned);
                    newly_orphaned.push(class_name.clone());
                }
            }
        }
        
        // Then process them to find affected children
        for orphaned_class in newly_orphaned {
            self.process_affected_children(&orphaned_class, &mut node_status);
        }
        
        // Add nodes for classes to remove
        for class_name in &self.classes_to_remove {
            if let Some(class) = game_data.classes.iter().find(|c| &c.name == class_name) {
                let node_id = graph.add_node(class.name.clone());
                added_nodes.insert(class.name.clone(), node_id);
                
                // Add parent if it exists and isn't being removed
                if let Some(parent) = &class.parent {
                    if !self.classes_to_remove.contains(parent) {
                        if let Some(parent_class) = game_data.classes.iter().find(|c| &c.name == parent) {
                            let parent_id = if let Some(&id) = added_nodes.get(parent) {
                                id
                            } else {
                                let id = graph.add_node(parent.clone());
                                added_nodes.insert(parent.clone(), id);
                                id
                            };
                            
                            graph.add_edge(parent_id, node_id, ());
                        }
                    }
                }
                
                // Add direct children
                if let Some(children) = self.dependency_cache.children.get(&class.name) {
                    for child_name in children {
                        if let Some(child_class) = game_data.classes.iter().find(|c| &c.name == child_name) {
                            let child_id = if let Some(&id) = added_nodes.get(child_name) {
                                id
                            } else {
                                let id = graph.add_node(child_name.clone());
                                added_nodes.insert(child_name.clone(), id);
                                id
                            };
                            
                            graph.add_edge(node_id, child_id, ());
                        }
                    }
                }
            }
        }
        
        // Add orphaned classes with their connections
        for class_name in &self.orphaned_classes {
            if let Some(class) = game_data.classes.iter().find(|c| &c.name == class_name) {
                let node_id = if let Some(&id) = added_nodes.get(&class.name) {
                    id
                } else {
                    let id = graph.add_node(class.name.clone());
                    added_nodes.insert(class.name.clone(), id);
                    id
                };
                
                // Add parent (which will be in the removal set)
                if let Some(parent) = &class.parent {
                    if let Some(parent_class) = game_data.classes.iter().find(|c| &c.name == parent) {
                        let parent_id = if let Some(&id) = added_nodes.get(parent) {
                            id
                        } else {
                            let id = graph.add_node(parent.clone());
                            added_nodes.insert(parent.clone(), id);
                            id
                        };
                        
                        graph.add_edge(parent_id, node_id, ());
                    }
                }
            }
        }
        
        // Add affected classes
        for class_name in &self.affected_classes {
            if let Some(class) = game_data.classes.iter().find(|c| &c.name == class_name) {
                let node_id = if let Some(&id) = added_nodes.get(&class.name) {
                    id
                } else {
                    let id = graph.add_node(class.name.clone());
                    added_nodes.insert(class.name.clone(), id);
                    id
                };
                
                // Add parent
                if let Some(parent) = &class.parent {
                    if let Some(parent_class) = game_data.classes.iter().find(|c| &c.name == parent) {
                        let parent_id = if let Some(&id) = added_nodes.get(parent) {
                            id
                        } else {
                            let id = graph.add_node(parent.clone());
                            added_nodes.insert(parent.clone(), id);
                            id
                        };
                        
                        graph.add_edge(parent_id, node_id, ());
                    }
                }
            }
        }
        
        // We cannot directly modify node properties in egui_graphs
        // Instead, we'll store node status in our state and use it when rendering

        self.graph = Some(graph);
    }
    
    // New method to process affected children separately to avoid borrow checker issues
    fn process_affected_children(&mut self, parent: &str, node_status: &mut HashMap<String, NodeStatus>) {
        // Get a copy of the children to avoid borrowing self mutably and immutably at the same time
        let children = if let Some(children) = self.dependency_cache.children.get(parent) {
            children.clone()
        } else {
            Vec::new()
        };
        
        // Now process the copied children list
        for child in children {
            if !self.classes_to_remove.contains(&child) && !self.orphaned_classes.contains(&child) {
                self.affected_classes.insert(child.clone());
                node_status.insert(child.clone(), NodeStatus::Affected);
                
                // Recursively mark children as affected
                self.process_affected_children(&child, node_status);
            }
        }
    }

    // Deprecated
    fn add_class_to_graph(
        &self,
        graph: &mut Graph<String, ()>,
        class: &GameDataClass,
        game_data: &GameDataClasses,
        depth: usize,
    ) {
        // Deprecated, use build_hierarchy instead
        if depth >= self.max_depth {
            return;
        }

        let class_node = graph.add_node(class.name.clone());

        let child_classes: Vec<&GameDataClass> = game_data.classes.iter()
            .filter(|c| c.parent.as_ref().map_or(false, |p| p == &class.name))
            .collect();

        for child_class in child_classes {
            let child_node = graph.add_node(child_class.name.clone());
            graph.add_edge(class_node, child_node, ());
            self.add_class_to_graph(graph, child_class, game_data, depth + 1);
        }
    }
    
    pub fn get_node_status(&self, class_name: &str) -> NodeStatus {
        if self.classes_to_remove.contains(class_name) {
            NodeStatus::Removed
        } else if self.orphaned_classes.contains(class_name) {
            NodeStatus::Orphaned
        } else if self.affected_classes.contains(class_name) {
            NodeStatus::Affected
        } else {
            NodeStatus::Normal
        }
    }

    // Add handler methods for code-behind pattern

    // Handler for class filter changes
    pub fn handle_class_filter_change(&mut self, game_data: &GameDataClasses) {
        self.update_filtered_classes(game_data);
    }

    // Handler for PBO filter changes 
    pub fn handle_pbo_filter_change(&mut self, game_data: &GameDataClasses) {
        self.update_filtered_pbos(game_data);
    }

    // Handler for root class selection
    pub fn handle_root_class_selection(&mut self, class_name: String) {
        self.root_class = class_name;
    }

    // Handler for PBO selection
    pub fn handle_pbo_selection(&mut self, pbo_path: &PathBuf) {
        // Any additional logic for PBO selection can be added here
    }

    // Handler for view mode changes
    pub fn handle_view_mode_change(&mut self, old_mode: &ViewMode) {
        if self.view_mode != *old_mode {
            // Reset graph when view mode changes
            self.graph = None;
            self.analysis_graph = None;
        }
    }

    // Handler for max depth changes
    pub fn handle_max_depth_change(&mut self, old_depth: usize) {
        if self.max_depth != old_depth {
            // Reset graph when depth changes
            self.graph = None;
        }
    }

    // Handler for hierarchy mode changes
    pub fn handle_hierarchy_mode_change(&mut self, old_mode: &HierarchyMode) {
        if self.hierarchy_mode != *old_mode {
            // Reset graph when hierarchy mode changes
            self.graph = None;
        }
    }

    // Handler for loading classes to remove
    pub fn handle_load_classes_to_remove_dialog(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Text files", &["txt"])
            .pick_file() 
        {
            match self.load_classes_to_remove(path) {
                Ok(_) => {
                    // Successfully loaded
                },
                Err(err) => {
                    eprintln!("Error loading classes: {}", err);
                }
            }
        }
    }

    // Handler for analyzing impact
    pub fn handle_analyze_impact(&mut self) {
        // This is a placeholder as it depends on having game_data
        // The actual graph update is triggered elsewhere
        self.graph = None; // Force graph refresh
    }

    // Handler for clearing impact analysis
    pub fn handle_clear_impact_analysis(&mut self) {
        self.classes_to_remove.clear();
        self.orphaned_classes.clear();
        self.affected_classes.clear();
        self.impact_file_path = None;
        self.graph = None;
    }

    // Handler for node selection
    pub fn handle_node_selection(&mut self, node_name: Option<String>) {
        self.selected_node = node_name;
    }
} 