import { Node, Link } from './types';

export type GraphData = {
  nodes: Node[];
  links: Link[];
};

export interface WebSocketMessage {
  action: string;
  data?: any;
}

export class WebSocketService {
  private static instance: WebSocketService | null = null;
  private socket: WebSocket | null = null;
  private listeners: Map<string, Function[]> = new Map();
  private reconnectDelay = 1000;
  private maxReconnectDelay = 30000;
  private reconnectTimer: number | null = null;
  private fixedUrl = 'ws://localhost:3000/ws';
  private isReady: boolean = false;
  private isProcessingRequest: boolean = false;
  private requestQueue: (() => void)[] = [];
  private currentRequestPromise: Promise<void> | null = null;
  private currentRequestResolve: (() => void) | null = null;

  constructor(private url: string) {
    // Enforce singleton pattern
    if (WebSocketService.instance) {
      console.log('WebSocketService instance already exists, returning existing instance');
      return WebSocketService.instance;
    }
    
    // Ignore the provided URL and always use the fixed one
    console.log(`Using fixed WebSocket URL: ${this.fixedUrl}`);
    WebSocketService.instance = this;
  }

  static getInstance(url: string): WebSocketService {
    if (!WebSocketService.instance) {
      new WebSocketService(url);
    }
    return WebSocketService.instance!;
  }

  connect(): void {
    // If already connected or connecting, don't create a new connection
    if (this.socket && (this.socket.readyState === WebSocket.OPEN || this.socket.readyState === WebSocket.CONNECTING)) {
      console.log('WebSocket is already connected or connecting');
      return;
    }

    try {
      // Update the UI with the URL
      const wsUrlElement = document.getElementById('ws-url');
      if (wsUrlElement) {
        wsUrlElement.textContent = `URL: ${this.fixedUrl}`;
      }
      
      console.log(`Connecting to WebSocket at ${this.fixedUrl}`);
      this.socket = new WebSocket(this.fixedUrl);

      this.socket.addEventListener('open', () => {
        console.log('WebSocket connection established successfully');
        this.reconnectDelay = 1000; // Reset reconnect delay on successful connection
        this.isReady = true;
        this.emit('connected', null);
        // Process any pending messages
        this.processNextMessage();
      });

      this.socket.addEventListener('message', (event) => {
        try {
          const data = JSON.parse(event.data);
          console.log('Received WebSocket message:', data);
          
          // Process message based on action
          if (data.action) {
            this.handleMessage(data);
          }
          
          // Mark request as complete
          if (this.currentRequestResolve) {
            this.currentRequestResolve();
            this.currentRequestResolve = null;
            this.currentRequestPromise = null;
          }
          
          // Process next request after receiving a response
          this.isProcessingRequest = false;
          this.processNextMessage();
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
          // Ensure we don't get stuck if there's an error
          if (this.currentRequestResolve) {
            this.currentRequestResolve();
            this.currentRequestResolve = null;
            this.currentRequestPromise = null;
          }
          this.isProcessingRequest = false;
          this.processNextMessage();
        }
      });

      this.socket.addEventListener('close', () => {
        console.log('WebSocket connection closed');
        this.isReady = false;
        this.isProcessingRequest = false;
        
        if (this.currentRequestResolve) {
          this.currentRequestResolve();
          this.currentRequestResolve = null;
          this.currentRequestPromise = null;
        }
        
        // Clear the queue on disconnect
        this.requestQueue = [];
        
        this.scheduleReconnect();
        this.emit('disconnected', null);
      });

      this.socket.addEventListener('error', (error) => {
        console.error('WebSocket error:', error);
        console.log('WebSocket connection failed. Please check:');
        console.log('1. Is the server running at the correct address?');
        console.log('2. Does the server have a WebSocket endpoint at this URL?');
        console.log(`3. Current URL: ${this.fixedUrl}`);
        
        // Ensure we don't get stuck if there's an error
        if (this.currentRequestResolve) {
          this.currentRequestResolve();
          this.currentRequestResolve = null;
          this.currentRequestPromise = null;
        }
        this.isProcessingRequest = false;
        this.processNextMessage();
        
        this.emit('error', error);
      });
    } catch (error) {
      console.error('Error establishing WebSocket connection:', error);
      this.scheduleReconnect();
    }
  }

  disconnect(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
    
    if (this.reconnectTimer !== null) {
      window.clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    
    this.isReady = false;
    this.isProcessingRequest = false;
    this.requestQueue = [];
    
    if (this.currentRequestResolve) {
      this.currentRequestResolve();
      this.currentRequestResolve = null;
      this.currentRequestPromise = null;
    }
  }

  resetConnectionAttempts(): void {
    this.reconnectDelay = 1000;
    console.log('Connection attempts reset');
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer !== null) {
      window.clearTimeout(this.reconnectTimer);
    }

    this.reconnectTimer = window.setTimeout(() => {
      console.log(`Attempting to reconnect in ${this.reconnectDelay}ms...`);
      this.connect();
      // Exponential backoff for reconnection
      this.reconnectDelay = Math.min(this.reconnectDelay * 1.5, this.maxReconnectDelay);
    }, this.reconnectDelay);
  }

  private processNextMessage(): void {
    if (!this.isReady || this.isProcessingRequest || this.requestQueue.length === 0) {
      return;
    }

    const nextRequest = this.requestQueue.shift();
    if (nextRequest) {
      this.isProcessingRequest = true;
      nextRequest();
    }
  }

  private queueRequest(action: string, data: any): Promise<void> {
    // If there's already a request in flight, wait for it to complete
    if (this.currentRequestPromise) {
      console.log('Request in flight, waiting...');
      return this.currentRequestPromise.then(() => this.queueRequest(action, data));
    }

    // Create a new promise for this request
    this.currentRequestPromise = new Promise((resolve) => {
      this.currentRequestResolve = resolve;
      
      const sendMessage = () => {
        const message: WebSocketMessage = { action, data };
        console.log('Sending message:', message);
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
          this.socket.send(JSON.stringify(message));
        } else {
          console.warn('Cannot send message, WebSocket is not open');
          if (this.currentRequestResolve) {
            this.currentRequestResolve();
            this.currentRequestResolve = null;
            this.currentRequestPromise = null;
          }
          this.isProcessingRequest = false;
        }
      };

      // Add to queue
      this.requestQueue.push(sendMessage);
      // Try to process next message
      this.processNextMessage();
    });

    return this.currentRequestPromise;
  }

  async send(action: string, payload: any): Promise<void> {
    if (!this.isReady) {
      console.log('WebSocket not ready, request will be queued');
    }
    return this.queueRequest(action, payload);
  }

  async requestGraphData(options: { root_class?: string, max_depth?: number } = {}): Promise<void> {
    return this.send('get_graph', options);
  }

  private handleMessage(message: WebSocketMessage): void {
    if (message.action && message.data) {

      if (message.action === 'graph_data') {

        if (message.data.success && message.data.data) {
          const graphData = message.data.data as GraphData;
          this.emit('graph_data', graphData);
        } else if (message.data.success === undefined) {

          const graphData = message.data as GraphData;
          this.emit('graph_data', graphData);
        } else {
          console.error('Server error:', message.data.error);
          this.emit('error', message.data.error);
        }
      } else if (message.action === 'error') {
        console.error('Server error:', message.data.error);
        this.emit('error', message.data.error);
      } else {

        this.emit(message.action, message.data);
      }
    }
  }

  on(event: string, callback: Function): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(callback);
  }

  off(event: string, callback: Function): void {
    if (!this.listeners.has(event)) {
      return;
    }
    
    const callbacks = this.listeners.get(event)!;
    const index = callbacks.indexOf(callback);
    
    if (index !== -1) {
      callbacks.splice(index, 1);
    }
  }

  private emit(event: string, data: any): void {
    if (this.listeners.has(event)) {
      this.listeners.get(event)!.forEach(callback => {
        try {
          callback(data);
        } catch (error) {
          console.error(`Error in ${event} event handler:`, error);
        }
      });
    }
  }
} 