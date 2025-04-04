import { Cosmograph } from '@cosmograph/cosmograph';
import { Node, Link } from './types';

export class CosmographRenderer {
  private cosmograph: Cosmograph<Node, Link>;
  private container: HTMLDivElement;
  private isSimulationPaused = false;
  private nodes: Node[] = [];
  private links: Link[] = [];
  private hoveredNodeId: string | null = null;
  private isHighlighted = false;

  constructor(containerId: string) {
    const element = document.getElementById(containerId);
    if (!element || !(element instanceof HTMLDivElement)) {
      throw new Error(`Container element with id ${containerId} not found or is not a div`);
    }
    this.container = element;

    // Define the configuration for Cosmograph
    const config = {
      // Appearance
      nodeColor: (d: any) => d.color || '#888885',
      nodeSize: (d: any) => d.size || 10,
      nodeLabelAccessor: (d: any) => d.label || d.id,
      linkColor: (d: any) => d.color || '#FFFFFF33',
      linkWidth: (d: any) => d.width || 1,
      backgroundColor: '#121212',
      showDynamicLabels: true,
      spaceSize: 8192, // Maximum space size for large graphs
      
      // Events
      onNodeMouseOver: (node: Node | undefined, index?: number, position?: [number, number]) => {
        if (node) {
          console.log('Node hovered:', node.id);
          this.hoveredNodeId = node.id;
          this.isHighlighted = true;
        }
      },
      onNodeMouseOut: () => {
        // Only reset if we're not in a clicked/selected state
        if (this.isHighlighted && !this.selectedNodeId) {
          console.log('Node hover ended');
          this.hoveredNodeId = null;
          this.isHighlighted = false;
          this.resetHighlighting();
        }
      },
      onSimulationTick: (alpha: number) => {
        // Update UI if needed on each tick
      },
      onSimulationEnd: () => {
        console.log('Simulation completed');
      }
    };

    // Create a Cosmograph instance with our configuration
    this.cosmograph = new Cosmograph(this.container, config);

    // Add event listeners for additional UI elements
    this.setupEventListeners();
  }

  // Track selected node separately from hovered node
  private selectedNodeId: string | null = null;

  private setupEventListeners(): void {
    // Node click event - using custom event handling since onNodeClick isn't in the type
    this.container.addEventListener('click', (event) => {
      // Clear any previous selection
      if (this.selectedNodeId) {
        this.selectedNodeId = null;
        // If we're not hovering over anything now, reset highlighting
        if (!this.hoveredNodeId) {
          this.resetHighlighting();
        }
        return;
      }
      
      // If we have a hovered node, select it on click
      if (this.hoveredNodeId) {
        const node = this.nodes.find(n => n.id === this.hoveredNodeId);
        if (node) {
          this.selectedNodeId = this.hoveredNodeId;
          this.cosmograph.zoomToNode(node);
        }
      }
    });

    // Add click handler to container background to reset highlighting
    this.container.addEventListener('mousedown', (event) => {
      if (event.target === this.container) {
        this.resetHighlighting();
        this.selectedNodeId = null;
        this.hoveredNodeId = null;
        this.isHighlighted = false;
      }
    });

    // Add handlers for buttons
    const fitViewButton = document.getElementById('fit-view');
    if (fitViewButton) {
      fitViewButton.addEventListener('click', () => {
        this.cosmograph.fitView();
        // Reset any highlighting when fitting view
        this.resetHighlighting();
        this.selectedNodeId = null;
        this.hoveredNodeId = null;
        this.isHighlighted = false;
      });
    }

    const toggleSimulationButton = document.getElementById('toggle-simulation');
    if (toggleSimulationButton) {
      toggleSimulationButton.addEventListener('click', () => {
        if (this.isSimulationPaused) {
          this.cosmograph.restart();
          toggleSimulationButton.textContent = 'Pause Simulation';
        } else {
          this.cosmograph.pause();
          toggleSimulationButton.textContent = 'Resume Simulation';
        }
        this.isSimulationPaused = !this.isSimulationPaused;
      });
    }

    // Add ESC key handler to reset highlighting and selection
    document.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        this.resetHighlighting();
        this.selectedNodeId = null;
        this.hoveredNodeId = null;
        this.isHighlighted = false;
      }
    });

    // Resize handler
    window.addEventListener('resize', this.handleResize.bind(this));
  }

  // Update the data in the graph
  setData(nodes: Node[], links: Link[]): void {
    // Store the data for later use
    this.nodes = nodes;
    this.links = links;

    // Set the data to the Cosmograph instance
    this.cosmograph.setData(nodes, links);
    
    // Start the simulation
    this.cosmograph.start();
    
    // Fit the view to show all nodes
    this.cosmograph.fitView();
    
    // Reset any highlighting or selection
    this.resetHighlighting();
    this.selectedNodeId = null;
    this.hoveredNodeId = null;
    this.isHighlighted = false;
    
    console.log(`Set ${nodes.length} nodes and ${links.length} links to graph`);
  }

  // Method to update config
  updateConfig(config: Partial<any>): void {
    this.cosmograph.setConfig(config);
  }

  // Method to highlight a node
  highlightNode(nodeId: string): void {
    const nodeToHighlight = this.nodes.find(node => node.id === nodeId);
    
    if (nodeToHighlight) {
      this.cosmograph.focusNode(nodeToHighlight);
      this.cosmograph.zoomToNode(nodeToHighlight);
      this.selectedNodeId = nodeId;
      this.isHighlighted = true;
    }
  }
  
  // Reset highlighting
  private resetHighlighting(): void {
    this.cosmograph.setConfig({
      nodeColor: (node: any) => node.color || '#88C6FF',
      linkColor: (link: any) => link.color || '#FFFFFF33',
      linkWidth: (link: any) => link.width || 1
    });
  }

  // Cleanup method
  destroy(): void {
    window.removeEventListener('resize', this.handleResize.bind(this));
    document.removeEventListener('keydown', this.handleResize.bind(this));
    this.cosmograph.remove();
  }

  private handleResize(): void {
    // The Cosmograph library handles resize events automatically
    // But we can add custom logic here if needed
  }
} 