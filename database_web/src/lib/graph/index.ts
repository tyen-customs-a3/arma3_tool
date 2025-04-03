import { Cosmograph, prepareCosmographData, CosmographData, CosmographDataPrepResult, CosmographConfig } from '@cosmograph/cosmograph';
import { GraphData, Node, Link } from '../../modules/types';

// Types specific to the graph module
export interface CosmographManagerOptions {
  container: HTMLDivElement;
  config: CosmographConfig;
}

// Node color constants
const NODE_COLORS = {
  normal: '#FFFFFF',
  removed: '#FF0000',
  orphaned: '#FFEB3B',
  affected: '#FFA500'
};

/**
 * Manages the Cosmograph visualization
 */
export class CosmographManager {
  private graph: Cosmograph | null = null;
  private currentGraphData: GraphData | null = null;
  private currentLinks: Link[] | null = null;
  private container: HTMLDivElement;
  private config: CosmographConfig;
  private preparedData: CosmographDataPrepResult<CosmographData> | null = null;

  constructor(options: CosmographManagerOptions) {
    this.container = options.container;
    this.config = {
      ...options.config,
      pointColor: NODE_COLORS.normal
    };
  }

  /**
   * Updates the graph with new data
   */
  public async updateGraph(graphData: GraphData) {
    console.log('Updating graph with data:', graphData);
    
    // Store the original data for UI purposes
    this.currentGraphData = graphData;
    
    // Store the links array for finding child nodes
    this.currentLinks = graphData.edges;
    
    // Print the count of nodes and edges
    console.log(`Nodes: ${graphData.nodes.length}, Edges: ${graphData.edges.length}`);
    
    try {
      // Prepare data for Cosmograph with label support
      const dataPrepConfig = {
        points: {
          pointIdBy: 'id',
          // Set the label column to use the 'name' property of nodes
          pointLabelBy: 'name',
          // Include necessary columns
          pointIncludeColumns: ['name', 'depth', 'parent_id', 'container_class', 'source_path', 'node_type']
        },
        links: {
          linkSourceBy: 'source',
          linkTargetsBy: ['target'],
        },
      };

      // Prepare the data - now types are compatible without conversion
      const result = await prepareCosmographData(dataPrepConfig, graphData.nodes, graphData.edges);
      
      if (!result) {
        console.error('Failed to prepare data for Cosmograph');
        return;
      }

      // Save prepared data for later use (e.g., when toggling labels)
      this.preparedData = result;
      const { points, links, cosmographConfig } = result;

      // Create or update Cosmograph
      if (!this.graph) {
        // Create a new Cosmograph instance with combined config
        this.graph = new Cosmograph(this.container, {
          points,
          links,
          ...cosmographConfig,
          ...this.config,
          // Ensure these are always set
          showLabels: true,
          pointLabelBy: 'name',
          // Handle click events
          onClick: (index: number | undefined, pointPosition: [number, number] | undefined, event: MouseEvent) => {
            if (this.config.onClick) {
              this.config.onClick(index, pointPosition, event);
            }
          }
        });
      } else {
        // If the graph already exists, update its config and data
        await this.graph.reset();
        await this.graph.setConfig({
          points,
          links,
          ...cosmographConfig,
          ...this.config,
          // Ensure these are always set
          showLabels: true,
          pointLabelBy: 'name',
          // Handle click events
          onClick: (index: number | undefined, pointPosition: [number, number] | undefined, event: MouseEvent) => {
            if (this.config.onClick) {
              this.config.onClick(index, pointPosition, event);
            }
          }
        });
      }

      // Start the simulation
      this.graph.restart();
      this.graph.start(1.0);
    } catch (error) {
      console.error('Error preparing data for Cosmograph:', error);
    }
  }

  /**
   * Selects a node by its ID
   */
  public selectNodeById(id: string) {
    if (!this.currentGraphData) return;
    
    const nodeIndex = this.currentGraphData.nodes.findIndex(n => n.id === id);
    if (nodeIndex !== -1) {
      console.log('Selecting node by index:', nodeIndex);
      this.graph?.selectPoint(nodeIndex);
      this.focusNodeByIndex(nodeIndex);
    }
  }

  /**
   * Focuses on a node
   */
  public focusNode(node: Node | undefined) {
    if (!node || !this.currentGraphData) {
      this.graph?.setFocusedPoint(undefined);
      return;
    }
    
    const nodeIndex = this.currentGraphData.nodes.indexOf(node);
    if (nodeIndex !== -1) {
      this.focusNodeByIndex(nodeIndex);
    }
  }

  /**
   * Focuses on a node by its index
   */
  private focusNodeByIndex(index: number) {
    console.log('Focusing node by index:', index);
    this.graph?.setFocusedPoint(index);
  }

  /**
   * Clears the current focus
   */
  public clearFocus() {
    this.graph?.setFocusedPoint(undefined);
  }

  /**
   * Zooms to a node by its ID
   */
  public zoomToNodeById(id: string) {
    if (!this.currentGraphData) return;
    
    const nodeIndex = this.currentGraphData.nodes.findIndex(n => n.id === id);
    if (nodeIndex !== -1) {
      console.log('Zooming to node by index:', nodeIndex);
      this.graph?.zoomToPoint(nodeIndex);
    }
  }

  /**
   * Finds child nodes for a given node ID
   */
  public findChildNodes(nodeId: string): string[] {
    if (!this.currentLinks) return [];
    const childNodes = this.currentLinks
      .filter(link => link.source === nodeId)
      .map(link => link.target);
    console.log('Found child nodes for', nodeId, ':', childNodes);
    return childNodes;
  }

  /**
   * Gets a node by its index
   */
  public getNodeByIndex(index: number): Node | undefined {
    return this.currentGraphData?.nodes[index];
  }

  /**
   * Gets the current graph data
   */
  public getCurrentGraphData(): GraphData | null {
    return this.currentGraphData;
  }

  /**
   * Gets a random point index
   */
  public getRandomPointIndex(): number {
    if (!this.currentGraphData?.nodes.length) return 0;
    return Math.floor(Math.random() * this.currentGraphData.nodes.length);
  }

  /**
   * Zooms to a point by its index
   */
  public zoomToPointByIndex(index: number) {
    this.graph?.zoomToPoint(index);
  }

  /**
   * Fits the view to show all nodes
   */
  public fitView() {
    this.graph?.fitView();
  }

  /**
   * Pauses the simulation
   */
  public pause() {
    this.graph?.pause();
  }

  /**
   * Starts the simulation
   */
  public start(alpha: number = 1.0) {
    this.graph?.start(alpha);
  }

  /**
   * Restarts the simulation
   */
  public async restart() {
    if (this.currentGraphData) {
      await this.updateGraph(this.currentGraphData);
    }
  }

  /**
   * Steps the simulation
   */
  public step() {
    this.graph?.step();
  }

  /**
   * Checks if the simulation is running
   */
  public isSimulationRunning(): boolean {
    return !!this.graph?.isSimulationRunning;
  }

  /**
   * Gets the simulation progress
   */
  public getSimulationProgress(): number | undefined {
    return this.graph?.progress;
  }

  /**
   * Sets the graph configuration
   */
  public async setConfig(config: CosmographConfig) {
    // Preserve our pointColor function and label settings when updating config
    this.config = {
      ...config,
      pointColor: NODE_COLORS.normal,
      // Always ensure labels are enabled
      showLabels: true,
      pointLabelBy: 'name'
    };
    if (this.graph && this.preparedData) {
      const { points, links, cosmographConfig } = this.preparedData;
      
      if (points && cosmographConfig) {
        await this.graph.setConfig({
          points,
          links,
          ...cosmographConfig,
          ...this.config
        });
      }
    }
  }

  /**
   * Shows labels for specific node IDs
   */
  public showLabelsForNodes(nodeIds: string[]) {
    if (this.graph && this.preparedData) {
      const { points, links, cosmographConfig } = this.preparedData;
      
      if (points && cosmographConfig) {
        const config = {
          ...this.config,
          showLabelsFor: nodeIds,
          points,
          links,
          ...cosmographConfig
        };
        this.graph.setConfig(config);
      }
    }
  }

  /**
   * Toggle label visibility
   */
  public toggleLabels(visible: boolean) {
    if (this.graph && this.preparedData) {
      const { points, links, cosmographConfig } = this.preparedData;
      
      if (points && cosmographConfig) {
        this.config = {
          ...this.config,
          showLabels: visible
        };
        
        // Make sure we include the points and links in the configuration update
        this.graph.setConfig({
          points,
          links,
          ...cosmographConfig,
          ...this.config
        });
      }
    }
  }
}

// Re-export types for convenience
export type { GraphData, Node, Link }; 