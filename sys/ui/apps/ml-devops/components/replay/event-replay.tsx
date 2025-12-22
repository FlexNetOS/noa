'use client';

/**
 * Event Replay UI - Replay saved event streams
 * 
 * Features:
 * - Load saved event streams from database
 * - Replay with adjustable speed
 * - Pause/resume functionality
 * - Visual progress indicator
 * 
 * Rust/Dioxus equivalent:
 * - use_coroutine for async replay logic
 * - tokio::time::interval for timing
 */

import { useState, useEffect } from 'react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useEventEmitter, useEventCount } from '@/lib/hooks/use-event-stream';
import { EventStore } from '@/lib/events/event-store';
import { getEventStream } from '@/lib/events/event-stream';
import {
  Play,
  Pause,
  RotateCcw,
  Save,
  FolderOpen,
  Download,
  Upload,
} from 'lucide-react';
import { Input } from '@/components/ui/input';

export function EventReplay({ className = '' }: { className?: string }) {
  const { emit } = useEventEmitter();
  const eventCount = useEventCount();
  const [isReplaying, setIsReplaying] = useState(false);
  const [replaySpeed, setReplaySpeed] = useState(100);
  const [savedStreams, setSavedStreams] = useState<any[]>([]);
  const [streamName, setStreamName] = useState('');
  const stream = getEventStream();
  const eventStore = new EventStore(stream);

  useEffect(() => {
    loadSavedStreams();
  }, []);

  const loadSavedStreams = async () => {
    try {
      const streams = await eventStore.list();
      setSavedStreams(streams ?? []);
    } catch (error) {
      console.error('Failed to load streams:', error);
    }
  };

  const handleSave = async () => {
    try {
      const name = streamName || `Stream ${new Date().toLocaleString()}`;
      await eventStore.save(name);
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'EVENT_STREAM_SAVED',
        timestamp: Date.now(),
        eventCount,
        filename: name,
      });
      setStreamName('');
      await loadSavedStreams();
    } catch (error) {
      console.error('Failed to save stream:', error);
    }
  };

  const handleLoad = async (id: string) => {
    try {
      const log = await eventStore.load(id);
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'EVENT_STREAM_LOADED',
        timestamp: Date.now(),
        eventCount: log?.events?.length ?? 0,
        filename: log?.name ?? '',
      });
      await loadSavedStreams();
    } catch (error) {
      console.error('Failed to load stream:', error);
    }
  };

  const handleReplay = async () => {
    setIsReplaying(true);
    emit({
      id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'REPLAY_STARTED',
      timestamp: Date.now(),
      eventCount,
    });

    try {
      await stream.replay(replaySpeed);
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'REPLAY_COMPLETED',
        timestamp: Date.now(),
      });
    } catch (error) {
      console.error('Replay error:', error);
    } finally {
      setIsReplaying(false);
    }
  };

  const handleReset = () => {
    stream.clear();
  };

  const handleExport = () => {
    eventStore.exportToFile(`event-stream-${Date.now()}.json`);
  };

  const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target?.files?.[0];
    if (file) {
      try {
        await eventStore.importFromFile(file);
        await loadSavedStreams();
      } catch (error) {
        console.error('Import error:', error);
      }
    }
  };

  return (
    <Card className={`p-6 ${className}`}>
      <div className="flex items-center gap-2 mb-6">
        <Play className="w-5 h-5 text-purple-500" />
        <h2 className="text-xl font-bold">Event Replay</h2>
      </div>

      <div className="space-y-6">
        {/* Event Count */}
        <div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
          <div className="text-sm text-gray-600 dark:text-gray-400">Total Events</div>
          <div className="text-3xl font-bold text-blue-600">{eventCount}</div>
        </div>

        {/* Replay Controls */}
        <div>
          <h3 className="text-sm font-semibold mb-2">Replay Controls</h3>
          <div className="space-y-2">
            <div>
              <label className="text-xs text-gray-600 dark:text-gray-400">
                Speed (ms delay): {replaySpeed}
              </label>
              <input
                type="range"
                min="10"
                max="500"
                value={replaySpeed}
                onChange={(e) => setReplaySpeed(Number(e.target.value))}
                className="w-full"
              />
            </div>
            <div className="flex gap-2">
              <Button
                onClick={handleReplay}
                disabled={isReplaying || eventCount === 0}
                className="flex-1 flex items-center gap-2"
              >
                {isReplaying ? (
                  <>
                    <Pause className="w-4 h-4" />
                    Replaying...
                  </>
                ) : (
                  <>
                    <Play className="w-4 h-4" />
                    Replay
                  </>
                )}
              </Button>
              <Button
                onClick={handleReset}
                variant="destructive"
                className="flex items-center gap-2"
              >
                <RotateCcw className="w-4 h-4" />
                Reset
              </Button>
            </div>
          </div>
        </div>

        {/* Save/Load */}
        <div>
          <h3 className="text-sm font-semibold mb-2">Persistence</h3>
          <div className="space-y-2">
            <div className="flex gap-2">
              <Input
                placeholder="Stream name (optional)"
                value={streamName}
                onChange={(e) => setStreamName(e.target.value)}
              />
              <Button onClick={handleSave} className="flex items-center gap-2">
                <Save className="w-4 h-4" />
                Save
              </Button>
            </div>
            <div className="flex gap-2">
              <Button
                onClick={handleExport}
                variant="outline"
                className="flex-1 flex items-center gap-2"
              >
                <Download className="w-4 h-4" />
                Export JSON
              </Button>
              <label className="flex-1">
                <Button
                  variant="outline"
                  className="w-full flex items-center gap-2"
                  asChild
                >
                  <span>
                    <Upload className="w-4 h-4" />
                    Import JSON
                  </span>
                </Button>
                <input
                  type="file"
                  accept=".json"
                  className="hidden"
                  onChange={handleImport}
                />
              </label>
            </div>
          </div>
        </div>

        {/* Saved Streams */}
        <div>
          <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
            <FolderOpen className="w-4 h-4" />
            Saved Streams ({savedStreams?.length ?? 0})
          </h3>
          <div className="space-y-1 max-h-48 overflow-y-auto">
            {savedStreams?.length > 0 ? (
              savedStreams.map((s) => (
                <div
                  key={s?.id}
                  className="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-900 rounded hover:bg-gray-100 dark:hover:bg-gray-800"
                >
                  <div className="text-sm">
                    <div className="font-medium">{s?.name}</div>
                    <div className="text-xs text-gray-500">
                      {s?.events?.length ?? 0} events
                    </div>
                  </div>
                  <Button
                    onClick={() => handleLoad(s?.id)}
                    size="sm"
                    variant="ghost"
                  >
                    Load
                  </Button>
                </div>
              ))
            ) : (
              <p className="text-sm text-gray-500 text-center py-4">
                No saved streams yet
              </p>
            )}
          </div>
        </div>
      </div>
    </Card>
  );
}
