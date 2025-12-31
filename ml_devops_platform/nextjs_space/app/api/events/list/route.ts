/**
 * List Event Streams API Route
 * 
 * Returns all saved event streams
 * 
 * Rust Translation:
 * - sqlx query with pagination support
 */

export const dynamic = 'force-dynamic';

import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/db';

export async function GET(request: NextRequest) {
  try {
    const eventLogs = await prisma.eventLog.findMany({
      orderBy: {
        createdAt: 'desc',
      },
      take: 50, // Limit to 50 most recent
    });

    return NextResponse.json(eventLogs);
  } catch (error) {
    console.error('List error:', error);
    return NextResponse.json(
      { error: 'Failed to list event streams' },
      { status: 500 }
    );
  }
}
