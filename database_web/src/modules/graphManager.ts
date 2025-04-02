import { Cosmograph } from '@cosmograph/cosmograph';
import { GraphData, Node, GraphConfig, Link } from './types';

export class GraphManager {
  private graph: Cosmograph<Node, Link> | null = null;
  private currentGraphData: GraphData | null = null;
  private currentLinks: Link[] | null = null;
  private container: HTMLDivElement;
  private config: GraphConfig;

  constructor(container: HTMLDivElement, config: GraphConfig) {
    this.container = container;
    this.config = config;
  }

  public updateGraph(graphData: GraphData) {
    // Store the original data for UI purposes
    this.currentGraphData = { 
      nodes: graphData.nodes,
      edges: graphData.edges
    };
    
    // Store the links array for finding child nodes
    this.currentLinks = graphData.edges;
    
    // Print the count of nodes and edges
    console.log(`Nodes: ${graphData.nodes.length}, Edges: ${graphData.edges.length}`);

    if (!this.graph) {
      this.graph = new Cosmograph<Node, Link>(this.container, this.config);
      this.graph.setData(graphData.nodes, graphData.edges);
    } else {
      this.graph.setConfig(this.config);
      this.graph.setData(graphData.nodes, graphData.edges);
    }

    this.graph.restart();
    this.graph.start(1.0);
  }

  public selectNodeById(id: string) {
    const node = this.currentGraphData?.nodes.find(n => n.id === id);
    if (node) {
      this.graph?.selectNode(node);
    }
  }

  public zoomToNodeById(id: string) {
    const node = this.currentGraphData?.nodes.find(n => n.id === id);
    if (node) {
      this.graph?.zoomToNode(node);
    }
  }

  public findChildNodes(nodeId: string): string[] {
    if (!this.currentLinks) return [];
    return this.currentLinks
      .filter(link => link.source === nodeId)
      .map(link => link.target);
  }

  public getNodeByIndex(index: number): Node | undefined {
    return this.currentGraphData?.nodes[index];
  }

  public getCurrentGraphData(): GraphData | null {
    return this.currentGraphData;
  }

  public getRandomPointIndex(): number {
    if (!this.currentGraphData?.nodes.length) return 0;
    return Math.floor(Math.random() * this.currentGraphData.nodes.length);
  }

  public selectPointByIndex(index: number) {
    const node = this.getNodeByIndex(index);
    if (node) {
      this.selectNodeById(node.id);
    }
  }

  public selectPointsByIndices(indices: number[]) {
    indices.forEach(index => this.selectPointByIndex(index));
  }

  public unselectPoints() {
    this.graph?.selectNodes([]);
  }

  public zoomToPointByIndex(index: number) {
    const node = this.getNodeByIndex(index);
    if (node) {
      this.zoomToNodeById(node.id);
    }
  }

  public fitView() {
    this.graph?.fitView();
  }

  public selectPointsInRange(_range: [[number, number], [number, number]]) {
    // Implementation will depend on new API
  }

  public pause() {
    this.graph?.pause();
  }

  public start(alpha: number = 1.0) {
    this.graph?.start(alpha);
  }

  public restart() {
    if (this.currentGraphData) {
      this.graph?.setData(this.currentGraphData.nodes, this.currentGraphData.edges);
      this.graph?.start(1.0);
    }
  }

  public step() {
    this.graph?.step();
  }

  public isSimulationRunning(): boolean {
    return this.graph?.isSimulationRunning || false;
  }

  public getSimulationProgress(): number | undefined {
    return this.graph?.progress;
  }

  public setConfig(config: GraphConfig) {
    this.config = config;
    this.graph?.setConfig(config);
  }
} 