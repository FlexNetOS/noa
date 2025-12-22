/**
 * API Route: /api/moe
 * MOE routing and execution endpoint
 */

import { NextRequest, NextResponse } from 'next/server';
import { getMOERouter } from '@/lib/moe/router';
import { getSharedResources } from '@/lib/moe/shared-resources';
import { ProviderType } from '@/lib/providers/provider-manager';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { action, messages, options, resourceType, resourceData } = body;

    const router = getMOERouter();
    const resources = getSharedResources();

    switch (action) {
      case 'route': {
        // Analyze and route without executing
        const decision = await router.route(messages, options);
        const analysis = router.analyzeTask(messages);
        return NextResponse.json({ decision, analysis });
      }

      case 'execute': {
        // Route and execute with fallback
        const result = await router.executeWithFallback(messages, options);
        return NextResponse.json(result);
      }

      case 'register': {
        // Register shared resource
        if (!resourceType || !resourceData) {
          return NextResponse.json({ error: 'resourceType and resourceData required' }, { status: 400 });
        }
        switch (resourceType) {
          case 'skill': resources.registerSkill(resourceData); break;
          case 'prompt': resources.registerPrompt(resourceData); break;
          case 'goal': resources.registerGoal(resourceData); break;
          case 'policy': resources.registerPolicy(resourceData); break;
          case 'rule': resources.registerRule(resourceData); break;
          case 'agent': resources.registerAgent(resourceData); break;
          case 'tool': resources.registerTool(resourceData); break;
          case 'command': resources.registerCommand(resourceData); break;
          case 'script': resources.registerScript(resourceData); break;
          case 'workflow': resources.registerWorkflow(resourceData); break;
          default:
            return NextResponse.json({ error: `Unknown resource type: ${resourceType}` }, { status: 400 });
        }
        return NextResponse.json({ success: true, resourceType, id: resourceData.id });
      }

      case 'stats': {
        return NextResponse.json(router.getStats());
      }

      default:
        return NextResponse.json({ error: 'Unknown action' }, { status: 400 });
    }
  } catch (error) {
    console.error('[/api/moe] Error:', error);
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Unknown error' },
      { status: 500 }
    );
  }
}

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const type = searchParams.get('type');
  const id = searchParams.get('id');

  const resources = getSharedResources();
  const router = getMOERouter();

  // Get specific resource by type and id
  if (type && id) {
    let resource;
    switch (type) {
      case 'skill': resource = resources.getSkill(id); break;
      case 'prompt': resource = resources.getPrompt(id); break;
      case 'goal': resource = resources.getGoal(id); break;
      case 'policy': resource = resources.getPolicy(id); break;
      case 'rule': resource = resources.getRule(id); break;
      case 'agent': resource = resources.getAgent(id); break;
      case 'tool': resource = resources.getTool(id); break;
      case 'command': resource = resources.getCommand(id); break;
      case 'script': resource = resources.getScript(id); break;
      case 'workflow': resource = resources.getWorkflow(id); break;
      default:
        return NextResponse.json({ error: 'Unknown type' }, { status: 400 });
    }
    return NextResponse.json({ [type]: resource || null });
  }

  // Get all resources of a type
  if (type) {
    let items;
    switch (type) {
      case 'skills': items = resources.getAllSkills(); break;
      case 'prompts': items = resources.getAllPrompts(); break;
      case 'goals': items = resources.getAllGoals(); break;
      case 'policies': items = resources.getAllPolicies(); break;
      case 'rules': items = resources.getAllRules(); break;
      case 'agents': items = resources.getAllAgents(); break;
      case 'tools': items = resources.getAllTools(); break;
      case 'commands': items = resources.getAllCommands(); break;
      case 'scripts': items = resources.getAllScripts(); break;
      case 'workflows': items = resources.getAllWorkflows(); break;
      case 'logs': items = resources.getLogs({ since: new Date(Date.now() - 3600000) }); break;
      case 'stats': return NextResponse.json(router.getStats());
      default:
        return NextResponse.json({ error: 'Unknown type' }, { status: 400 });
    }
    return NextResponse.json({ [type]: items });
  }

  // Return summary of all resources
  return NextResponse.json(resources.exportAll());
}
