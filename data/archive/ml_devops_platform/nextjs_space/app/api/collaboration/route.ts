import { NextRequest, NextResponse } from 'next/server';
import { getCollaborationManager } from '@/lib/collaboration/collaboration-manager';
import { Operation } from 'fast-json-patch';

/**
 * Collaboration API - HTTP-based collaboration for MVP
 * 
 * Note: For production, this should be upgraded to WebSocket.
 * Next.js API routes don't support WebSocket natively.
 * 
 * Production options:
 * 1. Use a separate WebSocket server (e.g., with Express + ws)
 * 2. Use Next.js custom server with WebSocket support
 * 3. Use a hosted real-time service (Pusher, Ably, etc.)
 * 4. Deploy with Rust backend (tokio-tungstenite WebSocket)
 * 
 * For MVP, we use HTTP long polling or Server-Sent Events
 */

const collaborationManager = getCollaborationManager();

/**
 * POST /api/collaboration - Send patches or join session
 */
export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { action, sessionId, userId, userName, targetId, patches } = body;

    switch (action) {
      case 'join': {
        // Join collaboration session
        const session = collaborationManager.joinSession(sessionId, {
          id: userId,
          name: userName,
          color: generateUserColor(userId),
          lastSeen: Date.now(),
        });

        return NextResponse.json({
          success: true,
          session: {
            id: session.id,
            users: Array.from(session.users.values()),
          },
        });
      }

      case 'leave': {
        // Leave session
        collaborationManager.leaveSession(sessionId, userId);
        return NextResponse.json({ success: true });
      }

      case 'send_patches': {
        // Broadcast patches
        const transformedPatches = collaborationManager.broadcastPatches(
          sessionId,
          userId,
          targetId,
          patches as Operation[]
        );

        return NextResponse.json({
          success: true,
          patches: transformedPatches,
        });
      }

      case 'update_cursor': {
        // Update cursor position
        const { position } = body;
        collaborationManager.updateCursor(sessionId, userId, position);
        return NextResponse.json({ success: true });
      }

      default:
        return NextResponse.json(
          { success: false, error: 'Unknown action' },
          { status: 400 }
        );
    }
  } catch (error) {
    console.error('Collaboration API error:', error);
    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Internal server error',
      },
      { status: 500 }
    );
  }
}

/**
 * GET /api/collaboration - Get session state or patches
 */
export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const action = searchParams.get('action');
    const sessionId = searchParams.get('sessionId');
    const userId = searchParams.get('userId');
    const since = searchParams.get('since');

    if (!sessionId) {
      return NextResponse.json(
        { success: false, error: 'sessionId required' },
        { status: 400 }
      );
    }

    switch (action) {
      case 'get_users': {
        // Get active users
        const users = collaborationManager.getActiveUsers(sessionId);
        return NextResponse.json({ success: true, users });
      }

      case 'get_patches': {
        // Get patches since timestamp
        const patches = collaborationManager.getSessionPatches(
          sessionId,
          since ? parseInt(since) : undefined
        );
        return NextResponse.json({ success: true, patches });
      }

      case 'poll': {
        // Long polling endpoint (simulates WebSocket)
        // In production, replace with actual WebSocket
        const users = collaborationManager.getActiveUsers(sessionId);
        const patches = collaborationManager.getSessionPatches(
          sessionId,
          since ? parseInt(since) : undefined
        );

        return NextResponse.json({
          success: true,
          users,
          patches,
          timestamp: Date.now(),
        });
      }

      default:
        return NextResponse.json(
          { success: false, error: 'Unknown action' },
          { status: 400 }
        );
    }
  } catch (error) {
    console.error('Collaboration API error:', error);
    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Internal server error',
      },
      { status: 500 }
    );
  }
}

/**
 * Generate consistent color for user
 */
function generateUserColor(userId: string): string {
  const colors = [
    '#FF6B6B', '#4ECDC4', '#45B7D1', '#F7B731', '#5F27CD',
    '#00D2D3', '#FF9FF3', '#54A0FF', '#48DBFB', '#1DD1A1',
  ];

  let hash = 0;
  for (let i = 0; i < userId.length; i++) {
    hash = userId.charCodeAt(i) + ((hash << 5) - hash);
  }

  return colors[Math.abs(hash) % colors.length];
}
