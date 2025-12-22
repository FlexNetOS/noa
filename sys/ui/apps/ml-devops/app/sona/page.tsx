/**
 * SONA Orchestration Dashboard
 * Interactive interface for creating and monitoring LLM workflows
 */

'use client';

import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Network, Layers, Activity, BookOpen, ArrowRight } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';
import { WorkflowBuilder } from '@/components/sona/workflow-builder';
import { WorkflowMonitor } from '@/components/sona/workflow-monitor';
import { WorkflowDefinition } from '@/lib/sona/types';
import { EXAMPLE_WORKFLOWS } from '@/lib/sona/workflows';

export default function SonaPage() {
  const [selectedWorkflow, setSelectedWorkflow] = useState<WorkflowDefinition | null>(null);
  const [activeTab, setActiveTab] = useState('overview');

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="space-y-2"
      >
        <div className="flex items-center gap-3">
          <div className="p-3 bg-primary/10 rounded-lg">
            <Network className="h-8 w-8 text-primary" />
          </div>
          <div>
            <h1 className="text-4xl font-bold">SONA Orchestration</h1>
            <p className="text-muted-foreground">
              Sequential Orchestration for Neural Agents
            </p>
          </div>
        </div>
      </motion.div>

      {/* Overview Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-1 md:grid-cols-3 gap-4"
      >
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Available Templates</CardTitle>
            <Layers className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">6</div>
            <p className="text-xs text-muted-foreground">
              Ready-to-use workflow templates
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Workflow Strategies</CardTitle>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">5</div>
            <p className="text-xs text-muted-foreground">
              Sequential, parallel, conditional, loop, map-reduce
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Agent Roles</CardTitle>
            <Network className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">5</div>
            <p className="text-xs text-muted-foreground">
              Planner, executor, reviewer, specialist, aggregator
            </p>
          </CardContent>
        </Card>
      </motion.div>

      {/* Main Content */}
      <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-4">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="builder">Workflow Builder</TabsTrigger>
          <TabsTrigger value="examples">Examples</TabsTrigger>
          <TabsTrigger value="docs">Documentation</TabsTrigger>
        </TabsList>

        {/* Overview Tab */}
        <TabsContent value="overview" className="space-y-4">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="grid grid-cols-1 lg:grid-cols-2 gap-6"
          >
            <Card>
              <CardHeader>
                <CardTitle>What is SONA?</CardTitle>
                <CardDescription>
                  Advanced LLM orchestration for complex multi-agent workflows
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <p className="text-sm text-muted-foreground">
                  SONA (Sequential Orchestration for Neural Agents) is a powerful system for
                  creating and executing complex LLM workflows with multiple specialized agents.
                </p>
                <div className="space-y-2">
                  <h4 className="font-medium text-sm">Key Features:</h4>
                  <ul className="text-sm text-muted-foreground space-y-1 list-disc list-inside">
                    <li>Multi-agent orchestration with specialized roles</li>
                    <li>Multiple execution strategies (sequential, parallel, etc.)</li>
                    <li>Real-time monitoring and event streaming</li>
                    <li>Built-in workflow templates</li>
                    <li>Advanced error handling and retries</li>
                    <li>Context and memory management</li>
                  </ul>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Workflow Strategies</CardTitle>
                <CardDescription>
                  Choose the right execution pattern for your use case
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-3">
                {[
                  {
                    name: 'Sequential',
                    desc: 'Execute steps one after another',
                    badge: 'Simple',
                  },
                  {
                    name: 'Parallel',
                    desc: 'Execute steps concurrently for speed',
                    badge: 'Fast',
                  },
                  {
                    name: 'Conditional',
                    desc: 'Execute based on runtime conditions',
                    badge: 'Dynamic',
                  },
                  {
                    name: 'Loop',
                    desc: 'Repeat steps with iteration',
                    badge: 'Iterative',
                  },
                  {
                    name: 'Map-Reduce',
                    desc: 'Parallel processing with aggregation',
                    badge: 'Scalable',
                  },
                ].map((strategy, i) => (
                  <div
                    key={strategy.name}
                    className="flex items-start justify-between p-3 border rounded-lg"
                  >
                    <div>
                      <div className="flex items-center gap-2">
                        <h5 className="font-medium text-sm">{strategy.name}</h5>
                        <Badge variant="outline" className="text-xs">
                          {strategy.badge}
                        </Badge>
                      </div>
                      <p className="text-xs text-muted-foreground mt-1">{strategy.desc}</p>
                    </div>
                  </div>
                ))}
              </CardContent>
            </Card>
          </motion.div>
        </TabsContent>

        {/* Builder Tab */}
        <TabsContent value="builder" className="space-y-4">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="space-y-4"
          >
            <WorkflowBuilder
              onWorkflowCreated={(workflow) => {
                setSelectedWorkflow(workflow);
              }}
            />

            {selectedWorkflow && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
              >
                <WorkflowMonitor workflow={selectedWorkflow} autoExecute={false} />
              </motion.div>
            )}
          </motion.div>
        </TabsContent>

        {/* Examples Tab */}
        <TabsContent value="examples" className="space-y-4">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="grid grid-cols-1 lg:grid-cols-2 gap-4"
          >
            {Object.entries(EXAMPLE_WORKFLOWS).map(([key, workflow]) => (
              <Card key={key} className="cursor-pointer hover:shadow-lg transition-shadow">
                <CardHeader>
                  <CardTitle className="flex items-center justify-between">
                    <span>{workflow.name}</span>
                    <Badge variant="outline">{workflow.strategy}</Badge>
                  </CardTitle>
                  <CardDescription>{workflow.description}</CardDescription>
                </CardHeader>
                <CardContent className="space-y-3">
                  <div className="flex gap-2 flex-wrap">
                    <Badge variant="secondary">
                      {workflow.steps.length} steps
                    </Badge>
                    <Badge variant="secondary">
                      {workflow.agents.length} agents
                    </Badge>
                  </div>
                  <div className="space-y-2">
                    <p className="text-sm font-medium">Agents:</p>
                    <div className="flex gap-2 flex-wrap">
                      {workflow.agents.map((agent) => (
                        <Badge key={agent.id} variant="outline" className="text-xs">
                          {agent.role}
                        </Badge>
                      ))}
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </motion.div>
        </TabsContent>

        {/* Documentation Tab */}
        <TabsContent value="docs" className="space-y-4">
          <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <BookOpen className="h-5 w-5" />
                  SONA Documentation
                </CardTitle>
              </CardHeader>
              <CardContent className="prose dark:prose-invert max-w-none">
                <h3>Getting Started</h3>
                <p>
                  SONA provides a powerful framework for orchestrating multi-agent LLM workflows.
                  Start by choosing a workflow template or building your own custom workflow.
                </p>

                <h3>Agent Roles</h3>
                <ul>
                  <li>
                    <strong>Planner:</strong> Breaks down complex tasks into manageable subtasks
                  </li>
                  <li>
                    <strong>Executor:</strong> Implements and executes specific tasks
                  </li>
                  <li>
                    <strong>Reviewer:</strong> Evaluates quality and provides feedback
                  </li>
                  <li>
                    <strong>Specialist:</strong> Provides domain-specific expertise
                  </li>
                  <li>
                    <strong>Aggregator:</strong> Synthesizes results from multiple sources
                  </li>
                </ul>

                <h3>Workflow Templates</h3>
                <ul>
                  <li>
                    <strong>Plan-Execute-Review:</strong> Classic three-phase workflow for
                    production-quality results
                  </li>
                  <li>
                    <strong>Multi-Expert Consensus:</strong> Leverage multiple specialists for
                    complex decisions
                  </li>
                  <li>
                    <strong>Iterative Refinement:</strong> Continuously improve results through
                    multiple iterations
                  </li>
                  <li>
                    <strong>Map-Reduce:</strong> Process large datasets in parallel with result
                    aggregation
                  </li>
                </ul>

                <h3>API Usage</h3>
                <pre className="bg-muted p-4 rounded-lg">
                  <code>{`// Execute a workflow
const result = await fetch('/api/sona', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    action: 'execute_template',
    workflowType: 'plan_execute_review',
    workflowConfig: {
      name: 'My Workflow',
      task: 'Build a web application'
    },
    input: {}
  })
});

const data = await result.json();
console.log(data.executionId, data.result);`}</code>
                </pre>
              </CardContent>
            </Card>
          </motion.div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
