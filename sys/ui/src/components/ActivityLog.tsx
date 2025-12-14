'use client';

import { useEffect, useState, useRef } from 'react';
import { wsClient, type WebSocketEvent } from '@/lib/websocket';
import { apiClient } from '@/lib/api';
import { Activity, AlertCircle, CheckCircle, Info, XCircle } from 'lucide-react';
import { cn } from '@/lib/utils';

interface ActivityEntry {
  id: string;
  type: string;
  message: string;
  timestamp: string;
  agent?: string;
}

type EntryType = 'info' | 'success' | 'warning' | 'error';

/**
 * Activity Log Component
 *
 * Displays real-time activity log entries from agents and system events.
 * Supports live scrolling and shows the last 10,000 entries.
 */
export default function ActivityLog() {
  const [entries, setEntries] = useState<ActivityEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const autoScrollRef = useRef(true);

  useEffect(() => {
    // Load initial entries
    const loadInitialEntries = async () => {
      try {
        const response = await apiClient.getActivityLog(1000);
        setEntries(response.entries.reverse()); // Show newest first
      } catch (error) {
        console.error('Failed to load activity log:', error);
      } finally {
        setLoading(false);
      }
    };

    loadInitialEntries();

    // Subscribe to WebSocket updates
    const unsubscribe = wsClient.on('activity', (event: WebSocketEvent) => {
      const newEntry = event.data as ActivityEntry;
      setEntries(prev => [newEntry, ...prev].slice(0, 10000)); // Keep last 10k
    });

    // Connect WebSocket if not already connected
    if (!wsClient.isConnected) {
      wsClient.connect();
    }

    return () => {
      unsubscribe();
    };
  }, []);

  // Auto-scroll to bottom when new entries arrive (if user hasn't scrolled up)
  useEffect(() => {
    if (autoScrollRef.current && scrollContainerRef.current) {
      scrollContainerRef.current.scrollTop = scrollContainerRef.current.scrollHeight;
    }
  }, [entries]);

  const handleScroll = () => {
    if (scrollContainerRef.current) {
      const { scrollTop, scrollHeight, clientHeight } = scrollContainerRef.current;
      // Enable auto-scroll if user is near bottom
      autoScrollRef.current = scrollTop + clientHeight >= scrollHeight - 100;
    }
  };

  const getEntryType = (type: string): EntryType => {
    if (type.includes('error') || type.includes('fail')) return 'error';
    if (type.includes('success') || type.includes('complete')) return 'success';
    if (type.includes('warning') || type.includes('warn')) return 'warning';
    return 'info';
  };

  const getIcon = (type: EntryType) => {
    switch (type) {
      case 'success':
        return <CheckCircle className="w-4 h-4 text-emerald-400" />;
      case 'error':
        return <XCircle className="w-4 h-4 text-red-400" />;
      case 'warning':
        return <AlertCircle className="w-4 h-4 text-yellow-400" />;
      default:
        return <Info className="w-4 h-4 text-blue-400" />;
    }
  };

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleTimeString();
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-slate-400">Loading activity log...</div>
      </div>
    );
  }

  return (
    <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl border border-slate-700 p-6">
      <div className="flex items-center gap-2 mb-4">
        <Activity className="w-5 h-5 text-blue-400" />
        <h2 className="text-xl font-semibold text-slate-100">Activity Log</h2>
        <span className="ml-auto text-sm text-slate-400">
          {entries.length} entries
        </span>
      </div>

      <div
        ref={scrollContainerRef}
        onScroll={handleScroll}
        className="h-96 overflow-y-auto space-y-2 pr-2"
      >
        {entries.length === 0 ? (
          <div className="text-center text-slate-400 py-8">
            No activity entries yet
          </div>
        ) : (
          entries.map((entry) => {
            const entryType = getEntryType(entry.type);
            return (
              <div
                key={entry.id}
                className={cn(
                  'flex items-start gap-3 p-3 rounded-lg bg-slate-900/50',
                  'hover:bg-slate-900/70 transition-colors'
                )}
              >
                {getIcon(entryType)}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="text-sm font-medium text-slate-200">
                      {entry.agent || 'System'}
                    </span>
                    <span className="text-xs text-slate-500">
                      {formatTimestamp(entry.timestamp)}
                    </span>
                  </div>
                  <p className="text-sm text-slate-300">{entry.message}</p>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}

