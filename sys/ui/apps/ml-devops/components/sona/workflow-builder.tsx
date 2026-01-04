/**
 * SONA Workflow Builder Component
 * Visual interface for creating and configsuring workflows
 */

'use client';

import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import
  {
    Plus,
    Trash2,
    Save,
    Play,
    Network,
    GitBranch,
    Repeat,
    Grid,
    Users,
    RotateCw,
  } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Textarea } from '@/components/ui/textarea';
import { Badge } from '@/components/ui/badge';
import { useSonaWorkflow } from '@/lib/hooks/use-sona';
import { WorkflowDefinition, WorkflowStep } from '@/lib/sona/types';

interface WorkflowBuilderProps
{
  onWorkflowCreated?: ( workflow: WorkflowDefinition ) => void;
}

export function WorkflowBuilder ( { onWorkflowCreated }: WorkflowBuilderProps )
{
  const [ workflowName, setWorkflowName ] = useState( '' );
  const [ workflowDescription, setWorkflowDescription ] = useState( '' );
  const [ strategy, setStrategy ] = useState<WorkflowDefinition[ 'strategy' ]>( 'sequential' );
  const [ templateType, setTemplateType ] = useState<string>( '' );
  const [ templateconfigs, setTemplateconfigs ] = useState<any>( {} );
  const [ showAdvanced, setShowAdvanced ] = useState( false );

  const { executeTemplate, isExecuting, result, error } = useSonaWorkflow();

  const strategies = [
    { value: 'sequential', label: 'Sequential', icon: GitBranch, description: 'Execute steps one after another' },
    { value: 'parallel', label: 'Parallel', icon: Network, description: 'Execute steps concurrently' },
    { value: 'conditional', label: 'Conditional', icon: Grid, description: 'Execute based on conditions' },
    { value: 'loop', label: 'Loop', icon: Repeat, description: 'Repeat steps with iteration' },
    { value: 'map_reduce', label: 'Map-Reduce', icon: Grid, description: 'Parallel map with reduction' },
  ];

  const templates = [
    { value: 'plan_execute_review', label: 'Plan-Execute-Review', icon: GitBranch },
    { value: 'consensus', label: 'Multi-Expert Consensus', icon: Users },
    { value: 'iterative_refinement', label: 'Iterative Refinement', icon: RotateCw },
    { value: 'map_reduce', label: 'Map-Reduce Processing', icon: Grid },
  ];

  const handleExecuteTemplate = async () =>
  {
    if ( !templateType ) return;

    const configs: any = {
      id: `workflow-${ Date.now() }`,
      name: workflowName || 'Untitled Workflow',
      ...templateconfigs,
    };

    try
    {
      const result = await executeTemplate( templateType, configs, {} );
      if ( onWorkflowCreated && result.status === 'completed' )
      {
        // Notify parent component
      }
    } catch ( err )
    {
      console.error( 'Workflow execution failed:', err );
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Network className="h-5 w-5" />
          Workflow Builder
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Basic Information */ }
        <div className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="workflow-name">Workflow Name</Label>
            <Input
              id="workflow-name"
              placeholder="Enter workflow name"
              value={ workflowName }
              onChange={ ( e ) => setWorkflowName( e.target.value ) }
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="workflow-description">Description</Label>
            <Textarea
              id="workflow-description"
              placeholder="Describe what this workflow does"
              value={ workflowDescription }
              onChange={ ( e ) => setWorkflowDescription( e.target.value ) }
              rows={ 3 }
            />
          </div>
        </div>

        {/* Template Selection */ }
        <div className="space-y-4">
          <Label>Workflow Template</Label>
          <div className="grid grid-cols-2 gap-3">
            { templates.map( ( template ) =>
            {
              const Icon = template.icon;
              return (
                <motion.div
                  key={ template.value }
                  whileHover={ { scale: 1.02 } }
                  whileTap={ { scale: 0.98 } }
                >
                  <Button
                    variant={ templateType === template.value ? 'default' : 'outline' }
                    className="w-full h-auto py-4 flex flex-col items-center gap-2"
                    onClick={ () =>
                    {
                      setTemplateType( template.value );
                      setTemplateconfigs( {} );
                    } }
                  >
                    <Icon className="h-6 w-6" />
                    <span className="text-sm font-medium">{ template.label }</span>
                  </Button>
                </motion.div>
              );
            } ) }
          </div>
        </div>

        {/* Template configsuration */ }
        <AnimatePresence mode="wait">
          { templateType && (
            <motion.div
              initial={ { opacity: 0, height: 0 } }
              animate={ { opacity: 1, height: 'auto' } }
              exit={ { opacity: 0, height: 0 } }
              className="space-y-4 border-t pt-4"
            >
              <Label>Template configsuration</Label>

              { templateType === 'plan_execute_review' && (
                <div className="space-y-2">
                  <Label htmlFor="task">Task Description</Label>
                  <Textarea
                    id="task"
                    placeholder="Describe the task to be planned, executed, and reviewed"
                    value={ templateconfigs.task || '' }
                    onChange={ ( e ) =>
                      setTemplateconfigs( { ...templateconfigs, task: e.target.value } )
                    }
                    rows={ 3 }
                  />
                </div>
              ) }

              { templateType === 'consensus' && (
                <div className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="consensus-task">Task Description</Label>
                    <Textarea
                      id="consensus-task"
                      placeholder="Describe the task for expert consensus"
                      value={ templateconfigs.task || '' }
                      onChange={ ( e ) =>
                        setTemplateconfigs( { ...templateconfigs, task: e.target.value } )
                      }
                      rows={ 3 }
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="specialist-count">Number of Specialists</Label>
                    <Input
                      id="specialist-count"
                      type="number"
                      min={ 2 }
                      max={ 10 }
                      value={ templateconfigs.specialistCount || 3 }
                      onChange={ ( e ) =>
                        setTemplateconfigs( {
                          ...templateconfigs,
                          specialistCount: parseInt( e.target.value ),
                        } )
                      }
                    />
                  </div>
                </div>
              ) }

              { templateType === 'iterative_refinement' && (
                <div className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="refinement-task">Task Description</Label>
                    <Textarea
                      id="refinement-task"
                      placeholder="Describe the task to refine iteratively"
                      value={ templateconfigs.task || '' }
                      onChange={ ( e ) =>
                        setTemplateconfigs( { ...templateconfigs, task: e.target.value } )
                      }
                      rows={ 3 }
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="max-iterations">Maximum Iterations</Label>
                    <Input
                      id="max-iterations"
                      type="number"
                      min={ 1 }
                      max={ 10 }
                      value={ templateconfigs.maxIterations || 3 }
                      onChange={ ( e ) =>
                        setTemplateconfigs( {
                          ...templateconfigs,
                          maxIterations: parseInt( e.target.value ),
                        } )
                      }
                    />
                  </div>
                </div>
              ) }

              { templateType === 'map_reduce' && (
                <div className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="map-tasks">Map Tasks (one per line)</Label>
                    <Textarea
                      id="map-tasks"
                      placeholder="Enter map tasks, one per line"
                      value={ ( templateconfigs.mapTasks || [] ).join( '\n' ) }
                      onChange={ ( e ) =>
                        setTemplateconfigs( {
                          ...templateconfigs,
                          mapTasks: e.target.value.split( '\n' ).filter( Boolean ),
                        } )
                      }
                      rows={ 4 }
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="reduce-task">Reduce Task</Label>
                    <Input
                      id="reduce-task"
                      placeholder="Describe how to combine results"
                      value={ templateconfigs.reduceTask || '' }
                      onChange={ ( e ) =>
                        setTemplateconfigs( {
                          ...templateconfigs,
                          reduceTask: e.target.value,
                        } )
                      }
                    />
                  </div>
                </div>
              ) }
            </motion.div>
          ) }
        </AnimatePresence>

        {/* Actions */ }
        <div className="flex gap-2 pt-4 border-t">
          <Button
            onClick={ handleExecuteTemplate }
            disabled={ !templateType || isExecuting }
            className="flex-1"
          >
            <Play className="h-4 w-4 mr-2" />
            { isExecuting ? 'Executing...' : 'Execute Workflow' }
          </Button>
        </div>

        {/* Result Display */ }
        { result && (
          <motion.div
            initial={ { opacity: 0, y: 20 } }
            animate={ { opacity: 1, y: 0 } }
            className="mt-4 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg"
          >
            <h4 className="font-medium text-green-900 dark:text-green-100 mb-2">
              Workflow Completed
            </h4>
            <p className="text-sm text-green-800 dark:text-green-200">
              Execution ID: { result.executionId }
            </p>
            <p className="text-sm text-green-800 dark:text-green-200">
              Duration: { result.duration }ms
            </p>
          </motion.div>
        ) }

        {/* Error Display */ }
        { error && (
          <motion.div
            initial={ { opacity: 0, y: 20 } }
            animate={ { opacity: 1, y: 0 } }
            className="mt-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg"
          >
            <h4 className="font-medium text-red-900 dark:text-red-100 mb-2">Error</h4>
            <p className="text-sm text-red-800 dark:text-red-200">{ error }</p>
          </motion.div>
        ) }
      </CardContent>
    </Card>
  );
}
