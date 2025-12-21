/**
 * Server-Sent Events (SSE) Route for Real-time Collaboration
 * 
 * Provides real-time updates using SSE (WebSocket alternative for App Router)
 * - Continuous connection for receiving updates
 * - Works with Next.js App Router
 * - Falls back when WebSocket not available
 */

export const dynamic = 'force-dynamic';
export const runtime = 'nodejs';

import { NextRequest } from 'next/server';
import { getCollaborationManager } from '@/lib/collaboration/collaboration-manager';

// In-memory event queues per session/user
const eventQueues = new Map<string, Map<string, Array<object>>>();

// Add event to user's queue (internal helper)
function queueEvent(sessionId: string, userId: string, event: object) {
  if (!eventQueues.has(sessionId)) {
    eventQueues.set(sessionId, new Map());
  }
  const session = eventQueues.get(sessionId)!;
  
  // Queue for all users except sender
  for (const [uid, queue] of session.entries()) {
    if (uid !== userId) {
      queue.push(event);
    }
  }
}

// SSE stream endpoint
export async function GET(request: NextRequest) {
  const url = new URL(request.url);
  const sessionId = url.searchParams.get('sessionId');
  const userId = url.searchParams.get('userId');

  if (!sessionId || !userId) {
    return new Response('Missing sessionId or userId', { status: 400 });
  }

  // Initialize queue for this user
  if (!eventQueues.has(sessionId)) {
    eventQueues.set(sessionId, new Map());
  }
  const sessionQueues = eventQueues.get(sessionId)!;
  if (!sessionQueues.has(userId)) {
    sessionQueues.set(userId, []);
  }

  const manager = getCollaborationManager();
  let lastPatchTime = Date.now();

  // Create SSE stream
  const stream = new ReadableStream({
    async start(controller) {
      const encoder = new TextEncoder();
      
      // Send initial connection event
      controller.enqueue(
        encoder.encode(`data: ${JSON.stringify({ type: 'connected', sessionId, userId })}\n\n`)
      );

      // Send current users
      const users = manager.getActiveUsers(sessionId);
      controller.enqueue(
        encoder.encode(`data: ${JSON.stringify({ type: 'users', users })}\n\n`)
      );

      // Polling loop for events
      const interval = setInterval(() => {
        try {
          // Check for queued events
          const queue = sessionQueues.get(userId);
          if (queue && queue.length > 0) {
            const events = queue.splice(0, queue.length);
            for (const event of events) {
              controller.enqueue(
                encoder.encode(`data: ${JSON.stringify(event)}\n\n`)
              );
            }
          }

          // Check for new patches
          const patches = manager.getSessionPatches(sessionId, lastPatchTime);
          if (patches.length > 0) {
            lastPatchTime = Date.now();
            controller.enqueue(
              encoder.encode(`data: ${JSON.stringify({ type: 'patches', patches })}\n\n`)
            );
          }

          // Send heartbeat every 30 seconds
          controller.enqueue(
            encoder.encode(`data: ${JSON.stringify({ type: 'heartbeat', timestamp: Date.now() })}\n\n`)
          );
        } catch (error) {
          console.error('SSE error:', error);
          clearInterval(interval);
          controller.close();
        }
      }, 100); // 100ms polling for low latency

      // Cleanup on close
      request.signal.addEventListener('abort', () => {
        clearInterval(interval);
        sessionQueues.delete(userId);
        manager.leaveSession(sessionId, userId);
        controller.close();
      });
    },
  });

  return new Response(stream, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache',
      'Connection': 'keep-alive',
    },
  });
}

// POST endpoint for sending events to other users
export async function POST(request: NextRequest) {
  const body = await request.json();
  const { sessionId, userId, type, data } = body;

  if (!sessionId || !userId) {
    return new Response('Missing sessionId or userId', { status: 400 });
  }

  const manager = getCollaborationManager();

  switch (type) {
    case 'patch':
      // Broadcast patch to session
      const patches = manager.broadcastPatches(sessionId, userId, data.targetId, data.patches);
      queueEvent(sessionId, userId, { type: 'patches', patches, from: userId });
      return Response.json({ success: true, patches });

    case 'cursor':
      // Update cursor position
      manager.updateCursor(sessionId, userId, data.position);
      queueEvent(sessionId, userId, { type: 'cursor', userId, position: data.position });
      return Response.json({ success: true });

    case 'presence':
      // Broadcast presence update
      queueEvent(sessionId, userId, { type: 'presence', userId, status: data.status });
      return Response.json({ success: true });

    default:
      return new Response('Unknown event type', { status: 400 });
  }
}
