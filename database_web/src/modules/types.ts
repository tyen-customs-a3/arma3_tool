import { CosmographInputConfig } from '@cosmograph/cosmograph';

export interface Node {
  id: string;
  name?: string;
  depth?: number;
  color?: string;
}

export interface Link {
  source: string;
  target: string;
  color?: string;
}

export interface GraphData {
  nodes: Node[];
  edges: Link[];
}

export interface WebSocketMessage {
  action: string;
  data: any;
}

export interface GraphConfig extends CosmographInputConfig<Node, Link> {
  simulation: {
    repulsion: number;
    gravity: number;
    linkSpring: number;
    linkDistance: number;
    decay: number;
    disabled: boolean;
  };
  renderLinks: boolean;
  linkWidth: number;
  linkColor: string;
  nodeColor: string;
  nodeSize: number;
  backgroundColor: string;
  curvedLinks: boolean;
  events: {
    onClick: (node: Node | undefined, index: number | undefined) => void;
  };
}

export interface NodeDetails {
  name: string;
  id: string;
  depth: string;
  children: string;
} 