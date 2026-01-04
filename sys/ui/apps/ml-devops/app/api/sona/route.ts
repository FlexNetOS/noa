/**
 * SONA Orchestration API Routes
 * HTTP endpoints for workflow execution and management
 */

import { NextRequest, NextResponse } from 'next/server';
import { getSonaOrchestrator } from '@/lib/sona/orchestrator';
import
  {
    WorkflowDefinition,
    OrchestrationResponse,
    SonaEvent,
  } from '@/lib/sona/types';
import
  {
    createSequentialWorkflow,
    createParallelWorkflow,
    createPlanExecuteReviewWorkflow,
    createMapReduceWorkflow,
    createConsensusWorkflow,
    createIterativeRefinementWorkflow,
    EXAMPLE_WORKFLOWS,
  } from '@/lib/sona/workflows';

const orchestrator = getSonaOrchestrator();

/**
 * POST /api/sona - Execute a workflow
 */
export async function POST ( request: NextRequest )
{
  try
  {
    const body = await request.json();
    const { action, workflow, input, workflowType, workflowconfigs } = body;

    switch ( action )
    {
      case 'execute': {
        if ( !workflow )
        {
          return NextResponse.json(
            { error: 'Workflow definition required' },
            { status: 400 }
          );
        }

        const result = await orchestrator.executeWorkflow(
          workflow as WorkflowDefinition,
          input || {}
        );

        return NextResponse.json( result );
      }

      case 'execute_template': {
        if ( !workflowType )
        {
          return NextResponse.json(
            { error: 'Workflow type required' },
            { status: 400 }
          );
        }

        let workflow: WorkflowDefinition;

        switch ( workflowType )
        {
          case 'sequential':
            workflow = createSequentialWorkflow(
              workflowconfigs?.id || `workflow-${ Date.now() }`,
              workflowconfigs?.name || 'Sequential Workflow',
              workflowconfigs?.steps || []
            );
            break;

          case 'parallel':
            workflow = createParallelWorkflow(
              workflowconfigs?.id || `workflow-${ Date.now() }`,
              workflowconfigs?.name || 'Parallel Workflow',
              workflowconfigs?.tasks || []
            );
            break;

          case 'plan_execute_review':
            workflow = createPlanExecuteReviewWorkflow(
              workflowconfigs?.id || `workflow-${ Date.now() }`,
              workflowconfigs?.name || 'Plan-Execute-Review Workflow',
              workflowconfigs?.task || ''
            );
            break;

          case 'map_reduce':
            workflow = createMapReduceWorkflow(
              workflowconfigs?.id || `workflow-${ Date.now() }`,
              workflowconfigs?.name || 'Map-Reduce Workflow',
              workflowconfigs?.mapTasks || [],
              workflowconfigs?.reduceTask || ''
            );
            break;

          case 'consensus':
            workflow = createConsensusWorkflow(
              workflowconfigs?.id || `workflow-${ Date.now() }`,
              workflowconfigs?.name || 'Consensus Workflow',
              workflowconfigs?.task || '',
              workflowconfigs?.specialistCount || 3
            );
            break;

          case 'iterative_refinement':
            workflow = createIterativeRefinementWorkflow(
              workflowconfigs?.id || `workflow-${ Date.now() }`,
              workflowconfigs?.name || 'Iterative Refinement Workflow',
              workflowconfigs?.task || '',
              workflowconfigs?.maxIterations || 3
            );
            break;

          default:
            return NextResponse.json(
              { error: `Unknown workflow type: ${ workflowType }` },
              { status: 400 }
            );
        }

        const result = await orchestrator.executeWorkflow( workflow, input || {} );
        return NextResponse.json( result );
      }

      case 'cancel': {
        const { executionId } = body;
        if ( !executionId )
        {
          return NextResponse.json(
            { error: 'Execution ID required' },
            { status: 400 }
          );
        }

        const cancelled = orchestrator.cancelExecution( executionId );
        return NextResponse.json( { success: cancelled } );
      }

      default:
        return NextResponse.json(
          { error: 'Invalid action' },
          { status: 400 }
        );
    }
  } catch ( error )
  {
    console.error( 'SONA API error:', error );
    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Internal server error',
      },
      { status: 500 }
    );
  }
}

/**
 * GET /api/sona - Get execution status or list workflows
 */
export async function GET ( request: NextRequest )
{
  try
  {
    const { searchParams } = new URL( request.url );
    const action = searchParams.get( 'action' );
    const executionId = searchParams.get( 'executionId' );

    switch ( action )
    {
      case 'status': {
        if ( !executionId )
        {
          return NextResponse.json(
            { error: 'Execution ID required' },
            { status: 400 }
          );
        }

        const execution = orchestrator.getExecution( executionId );
        if ( !execution )
        {
          return NextResponse.json(
            { error: 'Execution not found' },
            { status: 404 }
          );
        }

        return NextResponse.json( execution );
      }

      case 'context': {
        if ( !executionId )
        {
          return NextResponse.json(
            { error: 'Execution ID required' },
            { status: 400 }
          );
        }

        const context = orchestrator.getContext( executionId );
        if ( !context )
        {
          return NextResponse.json(
            { error: 'Context not found' },
            { status: 404 }
          );
        }

        // Convert Map to object for JSON serialization
        const serializedContext = {
          ...context,
          agentStates: Object.fromEntries( context.agentStates ),
        };

        return NextResponse.json( serializedContext );
      }

      case 'examples': {
        return NextResponse.json( {
          workflows: Object.keys( EXAMPLE_WORKFLOWS ),
          templates: [
            'sequential',
            'parallel',
            'plan_execute_review',
            'map_reduce',
            'consensus',
            'iterative_refinement',
          ],
        } );
      }

      default:
        return NextResponse.json(
          { error: 'Invalid action' },
          { status: 400 }
        );
    }
  } catch ( error )
  {
    console.error( 'SONA API error:', error );
    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Internal server error',
      },
      { status: 500 }
    );
  }
}
