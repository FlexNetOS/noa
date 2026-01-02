/**
 * API Route: /api/providers
 * Manages provider authentication and status
 */

import { NextRequest, NextResponse } from 'next/server';
import { getProviderManager, ProviderType, PROVIDER_CONFIGS } from '@/lib/providers/provider-manager';

export async function GET(request: NextRequest) {
  const manager = getProviderManager();
  const statuses = manager.getProviderStatuses();
  const activeProvider = manager.getActiveProvider();

  return NextResponse.json({
    providers: statuses,
    activeProvider,
    configs: PROVIDER_CONFIGS
  });
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { action, provider, credentials } = body;
    const manager = getProviderManager();

    switch (action) {
      case 'set_active':
        const success = manager.setActiveProvider(provider as ProviderType);
        return NextResponse.json({ success, activeProvider: manager.getActiveProvider() });

      case 'store_credentials':
        manager.setCredentials(provider as ProviderType, credentials);
        return NextResponse.json({ success: true });

      case 'check_status':
        const connected = manager.isProviderConnected(provider as ProviderType);
        return NextResponse.json({ provider, connected });

      default:
        return NextResponse.json({ error: 'Unknown action' }, { status: 400 });
    }
  } catch (error) {
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Unknown error' },
      { status: 500 }
    );
  }
}
