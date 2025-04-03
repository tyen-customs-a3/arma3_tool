import { WebSocketMessage, GraphQueryParams, ImpactAnalysisParams } from './types';

export class WebSocketService {
  private ws: WebSocket;
  private messageHandlers: Map<string, (data: any) => void>;
  private isReady: boolean = false;
  private isProcessingRequest: boolean = false;
  private requestQueue: (() => void)[] = [];
  private currentRequestPromise: Promise<void> | null = null;
  private currentRequestResolve: (() => void) | null = null;

  constructor(url: string) {
    this.ws = new WebSocket(url);
    this.messageHandlers = new Map();
    this.setupEventListeners();
  }

  private setupEventListeners() {
    this.ws.onopen = () => {
      console.log('Connected to WebSocket server');
      this.isReady = true;
      // Process any pending messages
      this.processNextMessage();
    };

    this.ws.onmessage = (event) => {
      try {
        console.log('Received message:', event.data);
        // Parse the message immediately
        const message: WebSocketMessage = JSON.parse(event.data);
        this.handleMessage(message);
        
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
        console.error('Error processing WebSocket message:', error);
        // Ensure we don't get stuck if there's an error
        if (this.currentRequestResolve) {
          this.currentRequestResolve();
          this.currentRequestResolve = null;
          this.currentRequestPromise = null;
        }
        this.isProcessingRequest = false;
        this.processNextMessage();
      }
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      // Ensure we don't get stuck if there's an error
      if (this.currentRequestResolve) {
        this.currentRequestResolve();
        this.currentRequestResolve = null;
        this.currentRequestPromise = null;
      }
      this.isProcessingRequest = false;
      this.processNextMessage();
    };

    this.ws.onclose = () => {
      console.log('Disconnected from WebSocket server');
      this.isReady = false;
      this.isProcessingRequest = false;
      if (this.currentRequestResolve) {
        this.currentRequestResolve();
        this.currentRequestResolve = null;
        this.currentRequestPromise = null;
      }
      // Clear the queue on disconnect
      this.requestQueue = [];
    };
  }

  private processNextMessage() {
    if (!this.isReady || this.isProcessingRequest || this.requestQueue.length === 0) {
      return;
    }

    const nextRequest = this.requestQueue.shift();
    if (nextRequest) {
      this.isProcessingRequest = true;
      nextRequest();
    }
  }

  private handleMessage(message: WebSocketMessage) {
    console.log('Handling message:', message);
    const handler = this.messageHandlers.get(message.action);
    if (handler) {
      if (message.action === 'graph_data') {
        if (message.data.success && message.data.data) {
          console.log('Processing graph data response:', message.data.data);
          handler(message.data.data);
        } else if (message.data.success === undefined) {
          // Direct graph data without response wrapper
          console.log('Processing direct graph data:', message.data);
          handler(message.data);
        } else {
          console.error('Error from server:', message.data.error);
        }
      } else if (message.action === 'database_response') {
        if (message.data.success && message.data.data) {
          const result = message.data.data;
          console.log('Processing database response:', result);
          // For impact analysis, we get graph_data inside the response
          if (result.graphData) {
            console.log('Found graph data in response:', result.graphData);
            const graphHandler = this.messageHandlers.get('graph_data');
            if (graphHandler) {
              graphHandler(result.graphData);
            }
          }
          handler(result);
        } else {
          console.error('Error from server:', message.data.error);
        }
      } else {
        handler(message.data);
      }
    } else {
      console.warn(`No handler found for action: ${message.action}`);
    }
  }

  public on(action: string, handler: (data: any) => void) {
    console.log('Registering handler for action:', action);
    this.messageHandlers.set(action, handler);
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
        this.ws.send(JSON.stringify(message));
      };

      // Add to queue
      this.requestQueue.push(sendMessage);
      // Try to process next message
      this.processNextMessage();
    });

    return this.currentRequestPromise;
  }

  public async send(action: string, data: any): Promise<void> {
    if (!this.isReady) {
      console.log('WebSocket not ready, request will be queued');
    }
    return this.queueRequest(action, data);
  }

  public async requestFullGraphData(params: GraphQueryParams): Promise<void> {
    return new Promise((resolve, reject) => {
      if (this.ws.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({
          action: 'get_full_graph',
          data: {
            exclude_source_patterns: params.excludeSourcePatterns || [],
            max_depth: params.maxDepth || 100,
            root_class: params.rootClass
          }
        }));
        resolve();
      } else {
        reject(new Error('WebSocket is not open'));
      }
    });
  }

  public async requestImpactAnalysis(params: ImpactAnalysisParams): Promise<void> {
    console.log('Requesting impact analysis with params:', params);
    return new Promise((resolve, reject) => {
      if (this.ws.readyState === WebSocket.OPEN) {
        const message = {
          action: 'database_query',
          data: {
            query_type: 'get_class_impact',
            parameters: {
              classes_to_remove: params.classesToRemove,
              exclude_patterns: params.excludeSourcePatterns || []
            }
          }
        };
        console.log('Sending impact analysis request:', message);
        this.ws.send(JSON.stringify(message));
        resolve();
      } else {
        reject(new Error('WebSocket is not open'));
      }
    });
  }
} 