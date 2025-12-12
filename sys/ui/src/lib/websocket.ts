/**
 * WebSocket Client for NOA real-time events
 */

export interface WebSocketEvent {
  type: string;
  data: unknown;
  timestamp: number;
}

type EventCallback = (event: WebSocketEvent) => void;

class WebSocketClient {
  private ws: WebSocket | null = null;
  private url: string;
  private listeners = new Map<string, Set<EventCallback>>();
  
  // Reconnection state
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private reconnecting = false;
  private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;

  constructor(url?: string) {
    this.url = url || process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws';
  }

  connect(): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      return;
    }

    try {
      this.ws = new WebSocket(this.url);

      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.reconnectAttempts = 0;
        this.reconnecting = false;
        this.clearReconnectTimeout();
      };

      this.ws.onmessage = (event) => {
        try {
          const message: WebSocketEvent = JSON.parse(event.data);
          this.emit(message.type, message);
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
        }
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.reconnect();
      };
    } catch (error) {
      console.error('Error connecting to WebSocket:', error);
      this.reconnect();
    }
  }

  private clearReconnectTimeout(): void {
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }
  }

  private reconnect(): void {
    if (this.reconnecting) {
      // Prevent overlapping reconnection attempts
      return;
    }

    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnect attempts reached');
      return;
    }

    this.reconnecting = true;
    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
    
    // Clear any existing reconnect timeout
    this.clearReconnectTimeout();
    
    this.reconnectTimeout = setTimeout(() => {
      console.log(`Attempting to reconnect... (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      this.reconnecting = false;
      this.reconnectTimeout = null;
      this.connect();
    }, delay);
  }

  on(eventType: string, callback: EventCallback): void {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, new Set());
    }
    this.listeners.get(eventType)!.add(callback);
  }

  off(eventType: string, callback: EventCallback): void {
    const callbacks = this.listeners.get(eventType);
    if (callbacks) {
      callbacks.delete(callback);
    }
  }

  private emit(eventType: string, event: WebSocketEvent): void {
    const callbacks = this.listeners.get(eventType);
    if (callbacks) {
      callbacks.forEach(callback => callback(event));
    }
  }

  send(eventType: string, data: unknown): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      const event: WebSocketEvent = {
        type: eventType,
        data,
        timestamp: Date.now(),
      };
      this.ws.send(JSON.stringify(event));
    } else {
      console.warn('WebSocket is not connected');
    }
  }

  disconnect(): void {
    // Clear any pending reconnect timeout
    this.clearReconnectTimeout();
    this.reconnecting = false;
    
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.listeners.clear();
  }
}

export const wsClient = new WebSocketClient();
