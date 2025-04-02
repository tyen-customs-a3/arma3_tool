import { GraphData, WebSocketMessage } from './types';

export class WebSocketService {
  private ws: WebSocket;
  private messageHandlers: Map<string, (data: any) => void>;
  private messageBuffer: string = '';
  private isProcessing: boolean = false;

  constructor(url: string) {
    this.ws = new WebSocket(url);
    this.messageHandlers = new Map();
    this.setupEventListeners();
  }

  private setupEventListeners() {
    this.ws.onopen = () => {
      console.log('Connected to WebSocket server');
    };

    this.ws.onmessage = (event) => {
      try {
        // Append new data to buffer
        this.messageBuffer += event.data;

        // Try to process complete messages
        this.processMessageBuffer();
      } catch (error) {
        console.error('Error processing WebSocket message:', error);
      }
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    this.ws.onclose = () => {
      console.log('Disconnected from WebSocket server');
    };
  }

  private processMessageBuffer() {
    if (this.isProcessing) return;

    try {
      this.isProcessing = true;

      // Try to parse the buffer as a complete message
      const message: WebSocketMessage = JSON.parse(this.messageBuffer);
      
      // Clear the buffer after successful parsing
      this.messageBuffer = '';

      // Process the message
      const handler = this.messageHandlers.get(message.action);
      if (handler) {
        // Convert data to match the new API structure
        if (message.action === 'graph_data') {
          const data = message.data as GraphData;
          handler(data);
        } else {
          handler(message.data);
        }
      } else {
        console.warn(`No handler found for action: ${message.action}`);
      }
    } catch (error) {
      // If parsing fails, it means we don't have a complete message yet
      // Keep the buffer for the next message
      console.log('Incomplete message received, waiting for more data...');
    } finally {
      this.isProcessing = false;
    }
  }

  public on(action: string, handler: (data: any) => void) {
    this.messageHandlers.set(action, handler);
  }

  public send(action: string, data: any) {
    const message: WebSocketMessage = { action, data };
    this.ws.send(JSON.stringify(message));
  }

  public requestGraphData(nodeCount: number, gridSize: number) {
    this.send('generate', {
      node_count: nodeCount,
      grid_size: gridSize
    });
  }

  public loadCsvFile(filePath: string) {
    console.log('Requesting CSV file load:', filePath);
    this.send('load_file', {
      path: filePath
    });
  }
} 