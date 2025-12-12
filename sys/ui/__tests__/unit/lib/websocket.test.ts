/**
 * WebSocket Client Unit Tests
 * 
 * Tests for the WebSocket client with focus on reconnection logic
 */

import { wsClient } from '@/lib/websocket';

describe('WebSocket Client', () => {
  let mockWebSocket: any;
  let wsInstances: any[] = [];

  beforeEach(() => {
    jest.clearAllMocks();
    jest.clearAllTimers();
    jest.useFakeTimers();
    
    // Reset instances array
    wsInstances = [];

    // Mock WebSocket constructor
    mockWebSocket = jest.fn().mockImplementation((url: string) => {
      const ws = {
        url,
        readyState: 0, // CONNECTING
        send: jest.fn(),
        close: jest.fn(),
        addEventListener: jest.fn(),
        removeEventListener: jest.fn(),
        onopen: null,
        onclose: null,
        onerror: null,
        onmessage: null,
      };
      
      // Store instance for later access
      wsInstances.push(ws);
      
      return ws;
    });

    // Replace global WebSocket with mock
    global.WebSocket = mockWebSocket as any;
    (global.WebSocket as any).CONNECTING = 0;
    (global.WebSocket as any).OPEN = 1;
    (global.WebSocket as any).CLOSING = 2;
    (global.WebSocket as any).CLOSED = 3;
    
    // Reset the wsClient singleton state by disconnecting and doing a successful connection cycle
    wsClient.disconnect();
    
    // Clear any pending mocks from the disconnect
    jest.clearAllMocks();
    
    // Do a quick connect/disconnect cycle to reset state
    wsClient.connect();
    if (wsInstances[0] && wsInstances[0].onopen) {
      wsInstances[0].onopen(); // This resets reconnectAttempts to 0
    }
    wsClient.disconnect();
    
    // Clear instances and mocks again for the actual test
    wsInstances = [];
    jest.clearAllMocks();
  });

  afterEach(() => {
    wsClient.disconnect();
    jest.useRealTimers();
  });

  describe('Reconnection Logic', () => {
    it('should prevent multiple concurrent reconnection attempts', () => {
      
      // Connect
      wsClient.connect();
      expect(mockWebSocket).toHaveBeenCalledTimes(1);

      // Simulate connection close which triggers reconnect
      const ws1 = wsInstances[0];
      if (ws1.onclose) {
        ws1.onclose();
      }

      // Simulate another close event before reconnect completes
      // This should be ignored due to reconnecting flag
      if (ws1.onclose) {
        ws1.onclose();
      }

      // Fast-forward time to trigger the reconnect
      jest.advanceTimersByTime(1000);

      // Should only have created 2 WebSocket instances:
      // 1. Initial connect
      // 2. First reconnect attempt
      // The second onclose should be ignored
      expect(mockWebSocket).toHaveBeenCalledTimes(2);
    });

    it('should clear pending reconnect timeout when disconnect is called', () => {
      
      // Connect
      wsClient.connect();
      
      // Simulate connection close which triggers reconnect
      const ws1 = wsInstances[0];
      if (ws1.onclose) {
        ws1.onclose();
      }

      // Verify reconnect is scheduled
      expect(jest.getTimerCount()).toBeGreaterThan(0);

      // Disconnect before reconnect fires
      wsClient.disconnect();

      // Fast-forward time
      jest.advanceTimersByTime(10000);

      // Should only have created 1 WebSocket (initial connect)
      // The scheduled reconnect should have been cancelled
      expect(mockWebSocket).toHaveBeenCalledTimes(1);
    });

    it('should clear reconnect timeout on successful connection', () => {
      
      // Connect
      wsClient.connect();
      expect(mockWebSocket).toHaveBeenCalledTimes(1);
      
      // Simulate connection close which triggers reconnect
      const ws1 = wsInstances[0];
      ws1.readyState = 3; // CLOSED - so next connect() doesn't return early
      if (ws1.onclose) {
        ws1.onclose();
      }

      // Fast-forward to trigger reconnect
      jest.advanceTimersByTime(1000);
      expect(mockWebSocket).toHaveBeenCalledTimes(2);

      // Simulate successful connection - this should reset reconnectAttempts
      const ws2 = wsInstances[1];
      if (ws2 && ws2.onopen) {
        ws2.readyState = 1; // OPEN
        ws2.onopen();
      }

      // Trigger another close - should start fresh reconnect logic
      ws2.readyState = 3; // CLOSED
      if (ws2 && ws2.onclose) {
        ws2.onclose();
      }

      // First attempt after reset should use base delay (1000ms)
      jest.advanceTimersByTime(1000);

      // Should have created 3 WebSocket instances:
      // 1. Initial connect
      // 2. First reconnect
      // 3. Second reconnect after successful connection (uses base delay since attempts reset)
      expect(mockWebSocket).toHaveBeenCalledTimes(3);
    });

    it('should use exponential backoff for reconnection delays', () => {
      
      // Connect
      wsClient.connect();
      const ws1 = wsInstances[0];
      ws1.readyState = 3; // CLOSED
      
      // First reconnect - should use base delay (1000ms)
      if (ws1.onclose) {
        ws1.onclose();
      }
      
      // Should not have created a new connection yet
      expect(mockWebSocket).toHaveBeenCalledTimes(1);
      
      // Advance almost to the delay
      jest.advanceTimersByTime(999);
      expect(mockWebSocket).toHaveBeenCalledTimes(1);
      
      // Advance the last millisecond - should trigger reconnect
      jest.advanceTimersByTime(1);
      expect(mockWebSocket).toHaveBeenCalledTimes(2);
      
      // Second reconnect - should use 2000ms delay (exponential: 2^1 * 1000)
      const ws2 = wsInstances[1];
      ws2.readyState = 3; // CLOSED
      
      if (ws2 && ws2.onclose) {
        ws2.onclose();
      }
      
      jest.advanceTimersByTime(1999);
      expect(mockWebSocket).toHaveBeenCalledTimes(2);
      
      jest.advanceTimersByTime(1);
      expect(mockWebSocket).toHaveBeenCalledTimes(3);
    });

    it('should stop reconnecting after max attempts', () => {
      const consoleErrorSpy = jest.spyOn(console, 'error').mockImplementation();
      
      // Connect
      wsClient.connect();
      
      // Trigger 5 reconnect attempts (max)
      for (let i = 0; i < 5; i++) {
        const ws = wsInstances[i];
        if (ws && ws.onclose) {
          ws.onclose();
        }
        jest.advanceTimersByTime(Math.pow(2, i) * 1000);
      }

      // Try to trigger one more reconnect - should be rejected
      const lastWs = wsInstances[wsInstances.length - 1];
      if (lastWs && lastWs.onclose) {
        lastWs.onclose();
      }
      jest.advanceTimersByTime(100000);

      // Should have error about max attempts
      expect(consoleErrorSpy).toHaveBeenCalledWith('Max reconnect attempts reached');
      
      consoleErrorSpy.mockRestore();
    });

    it('should reset reconnecting flag after reconnect attempt', () => {
      
      // Connect
      wsClient.connect();
      const ws1 = wsInstances[0];
      ws1.readyState = 3; // CLOSED
      
      // Trigger reconnect
      if (ws1.onclose) {
        ws1.onclose();
      }

      // Immediately try to trigger another - should be ignored
      if (ws1.onclose) {
        ws1.onclose();
      }

      // Fast-forward past the reconnect delay - this creates ws2
      jest.advanceTimersByTime(1000);

      // Now the reconnecting flag should be reset
      // Trigger another close on the new connection
      const ws2 = wsInstances[1];
      ws2.readyState = 3; // CLOSED
      
      if (ws2 && ws2.onclose) {
        ws2.onclose();
      }

      // This should schedule a new reconnect (with 2000ms delay for attempt #2)
      jest.advanceTimersByTime(2000);

      // Should have 3 WebSocket instances:
      // 1. Initial
      // 2. First reconnect
      // 3. Second reconnect (after flag was reset)
      expect(mockWebSocket).toHaveBeenCalledTimes(3);
    });
  });

  describe('Basic Functionality', () => {
    it('should connect to WebSocket', () => {
      wsClient.connect();
      
      expect(mockWebSocket).toHaveBeenCalled();
    });

    it('should not reconnect if already connected', () => {
      
      // First connect
      wsClient.connect();
      
      // Simulate successful connection
      const ws1 = wsInstances[0];
      if (ws1) {
        ws1.readyState = 1; // OPEN
      }
      
      // Try to connect again
      wsClient.connect();
      
      // Should only have called WebSocket constructor once
      expect(mockWebSocket).toHaveBeenCalledTimes(1);
    });

    it('should handle disconnect', () => {
      
      wsClient.connect();
      const ws = wsInstances[0];
      
      wsClient.disconnect();
      
      expect(ws.close).toHaveBeenCalled();
    });

    it('should send messages when connected', () => {
      
      wsClient.connect();
      const ws = wsInstances[0];
      ws.readyState = 1; // OPEN
      
      wsClient.send('test', { data: 'hello' });
      
      expect(ws.send).toHaveBeenCalled();
      const sentData = JSON.parse(ws.send.mock.calls[0][0]);
      expect(sentData.type).toBe('test');
      expect(sentData.data).toEqual({ data: 'hello' });
    });

    it('should not send messages when not connected', () => {
      const consoleWarnSpy = jest.spyOn(console, 'warn').mockImplementation();
      
      wsClient.connect();
      const ws = wsInstances[0];
      ws.readyState = 0; // CONNECTING
      
      wsClient.send('test', { data: 'hello' });
      
      expect(ws.send).not.toHaveBeenCalled();
      expect(consoleWarnSpy).toHaveBeenCalledWith('WebSocket is not connected');
      
      consoleWarnSpy.mockRestore();
    });
  });

  describe('Event Handling', () => {
    it('should register and trigger event listeners', () => {
      const callback = jest.fn();
      
      wsClient.on('test-event', callback);
      wsClient.connect();
      
      // Simulate message
      const ws = wsInstances[0];
      if (ws && ws.onmessage) {
        const mockEvent = {
          data: JSON.stringify({
            type: 'test-event',
            data: { message: 'hello' },
            timestamp: Date.now(),
          }),
        };
        ws.onmessage(mockEvent);
      }
      
      expect(callback).toHaveBeenCalled();
      expect(callback.mock.calls[0][0].type).toBe('test-event');
    });

    it('should unregister event listeners', () => {
      const callback = jest.fn();
      
      wsClient.on('test-event', callback);
      wsClient.off('test-event', callback);
      wsClient.connect();
      
      // Simulate message
      const ws = wsInstances[0];
      if (ws && ws.onmessage) {
        const mockEvent = {
          data: JSON.stringify({
            type: 'test-event',
            data: { message: 'hello' },
            timestamp: Date.now(),
          }),
        };
        ws.onmessage(mockEvent);
      }
      
      expect(callback).not.toHaveBeenCalled();
    });
  });
});
