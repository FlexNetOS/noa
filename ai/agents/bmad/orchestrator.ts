//! BMAD Orchestrator
//!
//! T362: Coordinates BMAD agents (analyst, architect, PO, dev) into a single flow.

import { BMADAnalystAgent } from './analyst';
import { BMADArchitectAgent } from './architect';
import { BMADProductOwnerAgent } from './po';
import { BMADDevAgent } from './dev';
import {
    BMADBacklogItem,
    BMADBlueprint,
    BMADContext,
    BMADRecommendation,
    BMADRisk,
    BMADWorkflowSnapshot,
} from './types';

export interface BMADOrchestrationResult {
    backlog: BMADBacklogItem[];
    blueprint: BMADBlueprint;
    risks: BMADRisk[];
    recommendations: BMADRecommendation[];
    decisions: string[];
    artifacts: string[];
}

export class BMADOrchestrator {
    private analyst: BMADAnalystAgent;
    private architect: BMADArchitectAgent;
    private po: BMADProductOwnerAgent;
    private dev: BMADDevAgent;

    constructor(
        analyst = new BMADAnalystAgent(),
        architect = new BMADArchitectAgent(),
        po = new BMADProductOwnerAgent(),
        dev = new BMADDevAgent(),
    ) {
        this.analyst = analyst;
        this.architect = architect;
        this.po = po;
        this.dev = dev;
    }

    async run(context: BMADContext): Promise<BMADOrchestrationResult> {
        const analysis = await this.analyst.analyze(context);
        const archPlan = this.architect.design(context, analysis.risks, analysis.discoveryBacklog);
        const prioritized = this.po.prioritize(archPlan.backlogUpdates, analysis.opportunities);
        const implementation = this.dev.prepareImplementation(archPlan.blueprint, prioritized.items);

        return {
            backlog: implementation.backlog,
            blueprint: archPlan.blueprint,
            risks: [...analysis.risks, ...archPlan.technicalRisks],
            recommendations: analysis.opportunities,
            decisions: archPlan.decisions,
            artifacts: implementation.artifacts,
        };
    }

    snapshot(context: BMADContext): Promise<BMADWorkflowSnapshot> {
        return this.run(context).then(result => ({
            findings: {
                risks: result.risks,
                recommendations: result.recommendations,
            },
            backlog: result.backlog,
            blueprint: result.blueprint,
            artifacts: result.artifacts,
        }));
    }
}
