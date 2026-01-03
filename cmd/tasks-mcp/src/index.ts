#!/usr/bin/env node
/**
 * @fileoverview NOA Tasks MCP Server
 * Model Context Protocol server for task management integration with Ruler
 */

import { FastMCP } from 'fastmcp';
import { z } from 'zod';

// Initialize the MCP server
const server = new FastMCP( {
    name: 'NOA Tasks Server',
    version: '1.0.0'
} );

// ============================================================================
// Task Management Tools
// ============================================================================

server.addTool( {
    name: 'list_tasks',
    description: 'List all tasks from the Ruler task management system. Returns tasks in various formats.',
    parameters: z.object( {
        status: z.enum( [ 'todo', 'in-progress', 'done', 'blocked', 'all' ] ).default( 'all' ).describe( 'Filter tasks by status' ),
        format: z.enum( [ 'csv', 'json', 'markdown' ] ).default( 'json' ).describe( 'Output format for task list' ),
        limit: z.number().default( 50 ).describe( 'Maximum number of tasks to return' )
    } ),
    execute: async ( args ) =>
    {
        // TODO: Integrate with Ruler CLI or API
        const tasks = [
            { id: '1', title: 'Sample Task', status: 'todo', priority: 'medium' }
        ];

        if ( args.format === 'csv' )
        {
            const header = 'id,title,status,priority';
            const rows = tasks.map( t => `${ t.id },${ t.title },${ t.status },${ t.priority }` );
            return [ header, ...rows ].join( '\n' );
        }

        if ( args.format === 'markdown' )
        {
            const header = '| ID | Title | Status | Priority |';
            const separator = '|---|---|---|---|';
            const rows = tasks.map( t => `| ${ t.id } | ${ t.title } | ${ t.status } | ${ t.priority } |` );
            return [ header, separator, ...rows ].join( '\n' );
        }

        return JSON.stringify( tasks, null, 2 );
    }
} );

server.addTool( {
    name: 'get_task',
    description: 'Get detailed information about a specific task by ID',
    parameters: z.object( {
        taskId: z.string().describe( 'The task ID to retrieve' )
    } ),
    execute: async ( args ) =>
    {
        // TODO: Integrate with Ruler CLI or API
        const task = {
            id: args.taskId,
            title: `Task ${ args.taskId }`,
            description: 'Task description from Ruler',
            status: 'todo',
            priority: 'medium',
            assignee: null,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
            subtasks: []
        };

        return JSON.stringify( task, null, 2 );
    }
} );

server.addTool( {
    name: 'create_task',
    description: 'Create a new task in the Ruler task management system',
    parameters: z.object( {
        title: z.string().describe( 'Task title' ),
        description: z.string().optional().describe( 'Task description' ),
        priority: z.enum( [ 'low', 'medium', 'high', 'critical' ] ).default( 'medium' ).describe( 'Task priority level' ),
        parentId: z.string().optional().describe( 'Parent task ID for subtasks' )
    } ),
    execute: async ( args ) =>
    {
        // TODO: Integrate with Ruler CLI or API
        const newTask = {
            id: `${ Date.now() }`,
            title: args.title,
            description: args.description ?? '',
            status: 'todo',
            priority: args.priority,
            parent_id: args.parentId ?? null,
            created_at: new Date().toISOString()
        };

        return `Created task: ${ JSON.stringify( newTask, null, 2 ) }`;
    }
} );

server.addTool( {
    name: 'update_task_status',
    description: 'Update the status of a task',
    parameters: z.object( {
        taskId: z.string().describe( 'The task ID to update' ),
        status: z.enum( [ 'todo', 'in-progress', 'done', 'blocked' ] ).describe( 'New status for the task' ),
        notes: z.string().optional().describe( 'Optional notes about the status change' )
    } ),
    execute: async ( args ) =>
    {
        // TODO: Integrate with Ruler CLI or API
        return `Updated task ${ args.taskId } status to: ${ args.status }${ args.notes ? ` (${ args.notes })` : '' }`;
    }
} );

// ============================================================================
// Spec/Plan Tools
// ============================================================================

server.addTool( {
    name: 'get_spec',
    description: 'Get the specification document for a task or project',
    parameters: z.object( {
        specPath: z.string().describe( 'Path to the spec file or spec ID' )
    } ),
    execute: async ( args ) =>
    {
        // TODO: Integrate with Ruler spec system
        return `Spec content for: ${ args.specPath }\n\n(Not yet implemented - integrate with Ruler)`;
    }
} );

server.addTool( {
    name: 'validate_plan',
    description: 'Validate a plan against the governing rules and spec',
    parameters: z.object( {
        planPath: z.string().describe( 'Path to the plan file' )
    } ),
    execute: async ( args ) =>
    {
        // TODO: Integrate with Ruler validation
        return `Validation result for: ${ args.planPath }\n\nStatus: PENDING\n(Not yet implemented - integrate with Ruler)`;
    }
} );

// ============================================================================
// Start the server
// ============================================================================

server.start( {
    transportType: 'stdio'
} ).catch( ( error: Error ) =>
{
    console.error( 'Failed to start MCP server:', error );
    process.exit( 1 );
} );
