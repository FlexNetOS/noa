/**
 * SONA Workflow Monitor Component
 * Real-time monitoring of workflow execution with event stream
 */

'use client';

import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Play,
  Pause,
  StopCircle,
  CheckCircle2,
  XCircle,
  Clock,
  Cpu,
  Network,
  Activity,
  ChevronRight,
  ChevronDown,
} from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator';
import { useSonaStream } from '@/lib/hooks/use-sona';
import { WorkflowDefinition, SonaEvent } from '@/lib/sona/types';

interface WorkflowMonitorProps {
  workflow: WorkflowDefinition;
  input?: Record<string, any>;
  autoExecute?: boolean;
}

export function WorkflowMonitor({
  workflow,
  input = {},
  autoExecute = false,
}: WorkflowMonitorProps) {
  const [expandedEvents, setExpandedEvents] = useState<Set<string>>(new Set());
  const { executeWithStream, isStreaming, events, error, result, clearEvents } =
    useSonaStream();

  React.useEffect(() => {
    if (autoExecute) {
      executeWithStream(workflow, input);
    }
  }, [autoExecute, workflow, input, executeWithStream]);

  const handleExecute = () => {
    clearEvents();
    executeWithStream(workflow, input);
  };

  const toggleEventExpansion = (eventId: string) => {
    const newExpanded = new Set(expandedEvents);
    if (newExpanded.has(eventId)) {
      newExpanded.delete(eventId);
    } else {
      newExpanded.add(eventId);
    }
    setExpandedEvents(newExpanded);
  };

  const getEventIcon = (type: string) => {
    switch (type) {
      case 'WORKFLOW_STARTED':
        return <Play className="h-4 w-4" />;
      case 'WORKFLOW_COMPLETED':
        return <CheckCircle2 className="h-4 w-4" />;
      case 'WORKFLOW_FAILED':
        return <XCircle className="h-4 w-4" />;
      case 'STEP_STARTED':
        return <Activity className="h-4 w-4" />;
      case 'STEP_COMPLETED':
        return <CheckCircle2 className="h-4 w-4" />;
      case 'STEP_FAILED':
        return <XCircle className="h-4 w-4" />;
      case 'AGENT_THINKING':
        return <Cpu className="h-4 w-4 animate-pulse" />;
      case 'AGENT_RESPONSE':
        return <Network className="h-4 w-4" />;
      default:
        return <Activity className="h-4 w-4" />;
    }
  };

  const getEventColor = (type: string) => {
    switch (type) {
      case 'WORKFLOW_STARTED':
      case 'STEP_STARTED':
        return 'bg-blue-500';
      case 'WORKFLOW_COMPLETED':
      case 'STEP_COMPLETED':
        return 'bg-green-500';
      case 'WORKFLOW_FAILED':
      case 'STEP_FAILED':
        return 'bg-red-500';
      case 'AGENT_THINKING':
        return 'bg-yellow-500';
      case 'AGENT_RESPONSE':
        return 'bg-purple-500';
      default:
        return 'bg-gray-500';
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Network className="h-5 w-5" />
            Workflow Monitor
          </CardTitle>
          <div className="flex items-center gap-2">
            {!isStreaming && !result && (
              <Button onClick={handleExecute} size="sm">
                <Play className="h-4 w-4 mr-2" />
                Execute
              </Button>
            )}
            {isStreaming && (
              <Badge variant="outline" className="animate-pulse">
                <Activity className="h-3 w-3 mr-1" />
                Running
              </Badge>
            )}
            {result && !isStreaming && (
              <Badge variant="outline" className="bg-green-50 dark:bg-green-900/20">
                <CheckCircle2 className="h-3 w-3 mr-1" />
                Completed
              </Badge>
            )}
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Workflow Info */}
        <div className="p-4 bg-muted rounded-lg space-y-2">
          <h4 className="font-medium">{workflow.name}</h4>
          <p className="text-sm text-muted-foreground">{workflow.description}</p>
          <div className="flex gap-2 mt-2">
            <Badge variant="secondary">{workflow.strategy}</Badge>
            <Badge variant="outline">{workflow.steps.length} steps</Badge>
            <Badge variant="outline">{workflow.agents.length} agents</Badge>
          </div>
        </div>

        {/* Events Timeline */}
        <div className="space-y-2">
          <h4 className="font-medium flex items-center gap-2">
            <Clock className="h-4 w-4" />
            Event Timeline
          </h4>
          <ScrollArea className="h-[400px] rounded-lg border p-4">
            <AnimatePresence mode="popLayout">
              {events.length === 0 && !isStreaming && (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  className="text-center text-muted-foreground py-8"
                >
                  <Activity className="h-8 w-8 mx-auto mb-2 opacity-50" />
                  <p>No events yet. Execute the workflow to see real-time updates.</p>
                </motion.div>
              )}

              {events.map((event, index) => {
                const isExpanded = expandedEvents.has(event.id);
                return (
                  <motion.div
                    key={event.id}
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: index * 0.05 }}
                    className="mb-4"
                  >
                    <div className="flex gap-3">
                      {/* Timeline indicator */}
                      <div className="flex flex-col items-center">
                        <div
                          className={`w-8 h-8 rounded-full ${getEventColor(
                            event.type
                          )} flex items-center justify-center text-white`}
                        >
                          {getEventIcon(event.type)}
                        </div>
                        {index < events.length - 1 && (
                          <div className="w-0.5 h-full bg-border mt-2" />
                        )}
                      </div>

                      {/* Event content */}
                      <div className="flex-1">
                        <div
                          className="cursor-pointer"
                          onClick={() => toggleEventExpansion(event.id)}
                        >
                          <div className="flex items-center gap-2 mb-1">
                            <h5 className="font-medium">
                              {event.type.replace(/_/g, ' ')}
                            </h5>
                            {isExpanded ? (
                              <ChevronDown className="h-4 w-4" />
                            ) : (
                              <ChevronRight className="h-4 w-4" />
                            )}
                          </div>
                          <p className="text-xs text-muted-foreground">
                            {new Date(event.timestamp).toLocaleTimeString()}
                          </p>
                        </div>

                        <AnimatePresence>
                          {isExpanded && (
                            <motion.div
                              initial={{ opacity: 0, height: 0 }}
                              animate={{ opacity: 1, height: 'auto' }}
                              exit={{ opacity: 0, height: 0 }}
                              className="mt-2 p-3 bg-muted rounded-lg text-sm overflow-auto"
                            >
                              <pre className="whitespace-pre-wrap">
                                {JSON.stringify(event.data, null, 2)}
                              </pre>
                            </motion.div>
                          )}
                        </AnimatePresence>
                      </div>
                    </div>
                  </motion.div>
                );
              })}
            </AnimatePresence>
          </ScrollArea>
        </div>

        {/* Result Display */}
        {result && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg space-y-2"
          >
            <h4 className="font-medium text-green-900 dark:text-green-100 flex items-center gap-2">
              <CheckCircle2 className="h-5 w-5" />
              Workflow Completed Successfully
            </h4>
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p className="text-green-800 dark:text-green-200">
                  <strong>Execution ID:</strong> {result.executionId}
                </p>
              </div>
              <div>
                <p className="text-green-800 dark:text-green-200">
                  <strong>Duration:</strong> {result.duration}ms
                </p>
              </div>
            </div>
            {result.result && (
              <div className="mt-2">
                <p className="text-sm font-medium text-green-900 dark:text-green-100 mb-1">
                  Result:
                </p>
                <ScrollArea className="h-32 bg-white dark:bg-gray-900 rounded p-2">
                  <pre className="text-xs text-green-800 dark:text-green-200">
                    {typeof result.result === 'string'
                      ? result.result
                      : JSON.stringify(result.result, null, 2)}
                  </pre>
                </ScrollArea>
              </div>
            )}
          </motion.div>
        )}

        {/* Error Display */}
        {error && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg"
          >
            <h4 className="font-medium text-red-900 dark:text-red-100 flex items-center gap-2">
              <XCircle className="h-5 w-5" />
              Workflow Failed
            </h4>
            <p className="text-sm text-red-800 dark:text-red-200 mt-2">{error}</p>
          </motion.div>
        )}
      </CardContent>
    </Card>
  );
}
