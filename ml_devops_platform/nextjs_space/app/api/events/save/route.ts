/**
 * Save Event Stream API Route
 * 
 * Persists event stream to database
 * 
 * Rust Translation:
 * - sqlx for PostgreSQL
 * - serde_json for serialization
 */

export const dynamic = 'force-dynamic';

import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/db';

export async function POST(request: NextRequest) {
  try {
    const { name, events } = await request.json();

    if (!name || !events) {
      return NextResponse.json(
        { error: 'Missing required fields' },
        { status: 400 }
      );
    }

    const eventLog = await prisma.eventLog.create({
      data: {
        name,
        events,
      },
    });

    return NextResponse.json(eventLog);
  } catch (error) {
    console.error('Save error:', error);
    return NextResponse.json(
      { error: 'Failed to save event stream' },
      { status: 500 }
    );
  }
}
