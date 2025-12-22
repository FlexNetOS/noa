/**
 * Delete Event Stream API Route
 * 
 * Removes saved event stream from database
 * 
 * Rust Translation:
 * - sqlx delete query
 */

export const dynamic = 'force-dynamic';

import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/db';

export async function DELETE(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const id = searchParams.get('id');

    if (!id) {
      return NextResponse.json(
        { error: 'Missing event log ID' },
        { status: 400 }
      );
    }

    await prisma.eventLog.delete({
      where: { id },
    });

    return NextResponse.json({ success: true });
  } catch (error) {
    console.error('Delete error:', error);
    return NextResponse.json(
      { error: 'Failed to delete event stream' },
      { status: 500 }
    );
  }
}
