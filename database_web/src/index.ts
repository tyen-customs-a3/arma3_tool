import { CosmographRenderer } from './cosmograph-renderer';
import { WebSocketService } from './websocket';
import { Node, Link } from './types';

// Define a sample dataset for initial rendering or testing
const sampleNodes: Node[] = [
  { id: 'node1', label: 'Node 1', color: 'red'},
  { id: 'node2', label: 'Node 2', color: 'green'},
  { id: 'node3', label: 'Node 3', color: 'yellow'},
  { id: 'node4', label: 'Node 4', color: 'brown'},
  { id: 'node5', label: 'Node 5', color: 'purple'},
  { id: 'node6', label: 'Node 6', color: 'blue'},
];

const sampleLinks: Link[] = [
  { source: 'node1', target: 'node2'},
  { source: 'node1', target: 'node3'},
  { source: 'node2', target: 'node4'},
  { source: 'node3', target: 'node5'},
  { source: 'node4', target: 'node6'},
  { source: 'node5', target: 'node6'},
  { source: 'node2', target: 'node5'},
];

document.addEventListener('DOMContentLoaded', () => {
  // Initialize the Cosmograph renderer
  const renderer = new CosmographRenderer('graph-container');
  
  // Setup WebSocket connection for real-time data
  // Try connecting directly to the server without the /api/ws path
  const wsUrl = 'ws://127.0.0.1:3000';
  
  const wsService = new WebSocketService(wsUrl);
  
  // For testing, we'll use the sample data with random colors
  renderer.setData(sampleNodes, sampleLinks);
  
  // Get UI elements
  const connectionStatus = document.getElementById('connection-status');
  const reconnectButton = document.getElementById('reconnect');
  const classesToPruneInput = document.getElementById('classes-to-prune') as HTMLInputElement;
  const analyzeImpactButton = document.getElementById('analyze-impact');
  
  // Update connection status UI
  const updateConnectionStatus = (connected: boolean) => {
    if (connectionStatus) {
      if (connected) {
        connectionStatus.textContent = 'Connected';
        connectionStatus.className = 'connection-status connected';
      } else {
        connectionStatus.textContent = 'Disconnected';
        connectionStatus.className = 'connection-status disconnected';
      }
    }
  };
  
  // Initially set as disconnected
  updateConnectionStatus(false);
  
  wsService.on('connected', () => {
    console.log('Connected to WebSocket server');
    updateConnectionStatus(true);
    wsService.requestGraphData();
  });
  
  wsService.on('graph_data', (data: { nodes: Node[], edges: Link[] }) => {
    console.log('Received graph data:', data);
    console.log('Nodes Count:', data.nodes.length);
    console.log('Links Count:', data.edges.length);
    if (data.nodes && data.edges) {
      // Use setData for the initial load to establish proper graph layout
      renderer.setData(data.nodes, data.edges);
    }
  });
  
  // We still need database_response handler for non-graph data
  wsService.on('database_response', (response: any) => {
    console.log('Received database response:', response);
    // Handle other types of database responses that aren't graph data
    // (statistics, success/failure messages, etc.)
  });
  
  wsService.on('error', (error: Event) => {
    console.error('WebSocket error:', error);
    // Fallback to sample data in case of connection error
    renderer.setData(sampleNodes, sampleLinks);
  });
  
  wsService.on('disconnected', () => {
    console.log('Disconnected from WebSocket server');
    updateConnectionStatus(false);
  });

  // Handle manual reconnect
  if (reconnectButton) {
    reconnectButton.addEventListener('click', () => {
      console.log('Manually reconnecting...');
      wsService.disconnect();
      wsService.resetConnectionAttempts();
      
      // Add a small visual feedback
      if (reconnectButton instanceof HTMLButtonElement) {
        const originalText = reconnectButton.textContent;
        reconnectButton.textContent = 'Connecting...';
        reconnectButton.disabled = true;
        
        setTimeout(() => {
          reconnectButton.textContent = originalText;
          reconnectButton.disabled = false;
          wsService.connect();
        }, 500); // Short delay before reconnecting
      } else {
        setTimeout(() => {
          wsService.connect();
        }, 500); // Short delay before reconnecting
      }
    });
  }

  // Connect to WebSocket
  wsService.connect();
  
  // Clean up event listeners on window unload
  window.addEventListener('beforeunload', () => {
    wsService.disconnect();
    renderer.destroy();
  });
}); 