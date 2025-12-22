/**
 * SONA Streaming API
 * Server-Sent Events (SSE) for real-time workflow execution updates
 */

import { NextRequest } from 'next/server';
import { getSonaOrchestrator } from '@/lib/sona/orchestrator';
import { WorkflowDefinition, SonaEvent } from '@/lib/sona/types';

/**
 * POST /api/sona/stream - Execute workflow with streaming updates
 */
export async function POST(request: NextRequest) {
  const orchestrator = getSonaOrchestrator();

  try {
    const body = await request.json();
    const { workflow, input } = body;

    if (!workflow) {
      return new Response(
        JSON.stringify({ error: 'Workflow definition required' }),
        {
          status: 400,
          headers: { 'Content-Type': 'application/json' },
        }
      );
    }

    // Create a readable stream for SSE
    const encoder = new TextEncoder();
    const stream = new ReadableStream({
      async start(controller) {
        // Subscribe to orchestration events
        const unsubscribe = orchestrator.onEvent((event: SonaEvent) => {
          try {
            const data = `data: ${JSON.stringify(event)}\n\n`;
            controller.enqueue(encoder.encode(data));
          } catch (error) {
            console.error('Error sending event:', error);
          }
        });

        try {
          // Execute workflow
          const result = await orchestrator.executeWorkflow(
            workflow as WorkflowDefinition,
            input || {}
          );

          // Send final result
          const finalData = `data: ${JSON.stringify({
            type: 'complete',
            result,
          })}\n\n`;
          controller.enqueue(encoder.encode(finalData));
        } catch (error) {
          // Send error
          const errorData = `data: ${JSON.stringify({
            type: 'error',
            error: error instanceof Error ? error.message : String(error),
          })}\n\n`;
          controller.enqueue(encoder.encode(errorData));
        } finally {
          // Cleanup
          unsubscribe();
          controller.close();
        }
      },
    });

    return new Response(stream, {
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        Connection: 'keep-alive',
      },
    });
  } catch (error) {
    console.error('SONA streaming error:', error);
    return new Response(
      JSON.stringify({
        error: error instanceof Error ? error.message : 'Internal server error',
      }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }
}
