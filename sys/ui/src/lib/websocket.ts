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
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private listeners = new Map<string, Set<EventCallback>>();

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

  private reconnect(): void {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnect attempts reached');
      return;
    }

    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
    
    setTimeout(() => {
      console.log(`Attempting to reconnect... (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
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

  get isConnected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
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
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.listeners.clear();
  }
}

export const wsClient = new WebSocketClient();
