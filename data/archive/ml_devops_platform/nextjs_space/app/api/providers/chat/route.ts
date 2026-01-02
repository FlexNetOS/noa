/**
 * API Route: /api/providers/chat
 * Unified chat endpoint that routes to the active provider
 */

import { NextRequest, NextResponse } from 'next/server';
import { getProviderManager, ProviderType } from '@/lib/providers/provider-manager';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { messages, provider, stream } = body;

    if (!messages || !Array.isArray(messages)) {
      return NextResponse.json(
        { error: 'messages array is required' },
        { status: 400 }
      );
    }

    const manager = getProviderManager();
    const targetProvider = provider as ProviderType || manager.getActiveProvider();

    // Check if provider is connected
    if (!manager.isProviderConnected(targetProvider)) {
      return NextResponse.json(
        { 
          error: `Provider ${targetProvider} is not connected`,
          requiresAuth: true,
          provider: targetProvider
        },
        { status: 401 }
      );
    }

    // Handle streaming response
    if (stream) {
      const result = await manager.chat(messages, { provider: targetProvider, stream: true });
      
      // Create SSE stream
      const encoder = new TextEncoder();
      const readableStream = new ReadableStream({
        async start(controller) {
          try {
            for await (const chunk of result as AsyncIterable<string>) {
              const data = JSON.stringify({ content: chunk });
              controller.enqueue(encoder.encode(`data: ${data}\n\n`));
            }
            controller.enqueue(encoder.encode('data: [DONE]\n\n'));
            controller.close();
          } catch (error) {
            controller.error(error);
          }
        }
      });

      return new Response(readableStream, {
        headers: {
          'Content-Type': 'text/event-stream',
          'Cache-Control': 'no-cache',
          'Connection': 'keep-alive'
        }
      });
    }

    // Non-streaming response
    const result = await manager.chat(messages, { provider: targetProvider, stream: false });
    
    return NextResponse.json({
      provider: targetProvider,
      content: result,
      timestamp: new Date().toISOString()
    });

  } catch (error) {
    console.error('[/api/providers/chat] Error:', error);
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Unknown error' },
      { status: 500 }
    );
  }
}
