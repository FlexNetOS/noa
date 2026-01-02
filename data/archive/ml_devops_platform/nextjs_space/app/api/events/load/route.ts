/**
 * Load Event Stream API Route
 * 
 * Retrieves saved event stream from database
 * 
 * Rust Translation:
 * - sqlx query with serde deserialization
 */

export const dynamic = 'force-dynamic';

import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/db';

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const id = searchParams.get('id');

    if (!id) {
      return NextResponse.json(
        { error: 'Missing event log ID' },
        { status: 400 }
      );
    }

    const eventLog = await prisma.eventLog.findUnique({
      where: { id },
    });

    if (!eventLog) {
      return NextResponse.json(
        { error: 'Event log not found' },
        { status: 404 }
      );
    }

    return NextResponse.json(eventLog);
  } catch (error) {
    console.error('Load error:', error);
    return NextResponse.json(
      { error: 'Failed to load event stream' },
      { status: 500 }
    );
  }
}
