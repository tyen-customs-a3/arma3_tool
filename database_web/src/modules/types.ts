export interface Node {
  id: string;
  name?: string;
  depth?: number;
  color?: string;
  parent_id?: string;
  container_class?: string;
  source_path?: string;
  node_type?: 'normal' | 'removed' | 'orphaned' | 'affected';
  [key: string]: unknown;
}

export interface Link {
  source: string;
  target: string;
  color?: string;
  [key: string]: unknown;
}

export interface GraphData {
  nodes: Node[];
  edges: Link[];
}

export interface WebSocketMessage {
  action: string;
  data: any;
}

export interface GraphQueryParams {
  excludeSourcePatterns?: string[];  // Patterns to exclude from source paths
  maxDepth?: number;                 // Maximum depth to traverse
  rootClass?: string;                // Optional root class to start from
}

export interface ImpactAnalysisParams {
  classesToRemove: string[];
  excludeSourcePatterns?: string[];
}

export interface ImpactAnalysisResult {
  removedClasses: string[];
  orphanedClasses: string[];
  affectedClasses: string[];
  graphData: GraphData;
}

export interface NodeDetails {
  name: string;
  id: string;
  depth: string;
  children: string;
  parent: string;
  container: string;
  source: string;
} 