import { NextRequest, NextResponse } from 'next/server';
import { getServerSession } from 'next-auth';
import { authOptions } from '@/lib/auth';
import { prisma } from '@/lib/db';

export const dynamic = 'force-dynamic';

export async function GET(req: NextRequest) {
  try {
    const session = await getServerSession(authOptions);

    if (!session?.user) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    // Check if user is admin
    const user = await prisma.user.findUnique({
      where: { email: session.user.email! },
    });

    if (user?.role !== 'admin') {
      return NextResponse.json({ error: 'Forbidden - Admin access required' }, { status: 403 });
    }

    // Get OAuth config for Google
    const config = await prisma.oAuthConfig.findUnique({
      where: { provider: 'google' },
    });

    if (!config) {
      return NextResponse.json({ error: 'OAuth configuration not found' }, { status: 404 });
    }

    if (!config.enabled) {
      return NextResponse.json({ error: 'OAuth is disabled' }, { status: 400 });
    }

    // Basic validation
    if (!config.clientId || !config.clientSecret) {
      return NextResponse.json({ error: 'OAuth configuration is incomplete' }, { status: 400 });
    }

    // Validate Client ID format
    if (!config.clientId.endsWith('.apps.googleusercontent.com')) {
      return NextResponse.json(
        { error: 'Invalid Client ID format. Should end with .apps.googleusercontent.com' },
        { status: 400 }
      );
    }

    return NextResponse.json({
      success: true,
      message: 'OAuth configuration is valid',
      provider: 'google',
      configured: true,
    });
  } catch (error) {
    console.error('Error testing OAuth config:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
