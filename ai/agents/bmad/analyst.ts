//! BMAD Analyst Agent
//!
//! T363: Implements the discovery and analysis role for BMAD.

import { BMADBacklogItem, BMADContext, BMADRecommendation, BMADRisk } from './types';

export interface AnalystFindings {
    risks: BMADRisk[];
    opportunities: BMADRecommendation[];
    discoveryBacklog: BMADBacklogItem[];
}

export class BMADAnalystAgent {
    private readonly now: Date;

    constructor(now: Date = new Date()) {
        this.now = now;
    }

    async analyze(context: BMADContext): Promise<AnalystFindings> {
        const risks = this.deriveRisks(context);
        const opportunities = this.deriveRecommendations(context);
        const discoveryBacklog = this.seedBacklog(context, risks);

        return { risks, opportunities, discoveryBacklog };
    }

    private deriveRisks(context: BMADContext): BMADRisk[] {
        const risks: BMADRisk[] = [];

        if (!context.techStack || context.techStack.length === 0) {
            risks.push({
                id: 'risk-tech-uncertainty',
                description: 'Tech stack not defined; velocity and quality at risk.',
                impact: 'medium',
                likelihood: 'medium',
                mitigation: 'Select provisional stack and validate with a spike.',
                owner: 'analyst',
            });
        }

        if ((context.timelineWeeks || 0) < 6) {
            risks.push({
                id: 'risk-schedule',
                description: 'Timeline is aggressive for the scope described.',
                impact: 'high',
                likelihood: 'medium',
                mitigation: 'Reduce scope to MVP and timebox experiments.',
                owner: 'analyst',
            });
        }

        if (context.constraints.length > 0) {
            risks.push({
                id: 'risk-constraints',
                description: `Found ${context.constraints.length} constraint(s) that may block delivery.`,
                impact: 'medium',
                likelihood: 'high',
                mitigation: 'Map each constraint to a mitigation and owner.',
                owner: 'analyst',
            });
        }

        return risks;
    }

    private deriveRecommendations(context: BMADContext): BMADRecommendation[] {
        const recs: BMADRecommendation[] = [
            {
                summary: 'Align stakeholders on success metrics',
                details: [
                    'Capture 3-5 measurable outcomes tied to objectives.',
                    'Agree on leading indicators the agents can track automatically.',
                ],
                owner: 'product',
                priority: 'p0',
            },
        ];

        if (!context.dependencies || context.dependencies.length === 0) {
            recs.push({
                summary: 'Document upstream/downstream dependencies',
                details: [
                    'Identify data producers/consumers.',
                    'Add availability SLOs and failure modes.',
                ],
                owner: 'architect',
                priority: 'p1',
            });
        }

        recs.push({
            summary: 'Establish daily risk and decision log',
            details: [
                `Start logging immediately (${this.now.toISOString()}) for auditability.`,
                'Include owner, timestamp, and mitigation for each entry.',
            ],
            owner: 'po',
            priority: 'p1',
        });

        return recs;
    }

    private seedBacklog(context: BMADContext, risks: BMADRisk[]): BMADBacklogItem[] {
        const items: BMADBacklogItem[] = [];

        items.push({
            id: 'bmad-disc-01',
            title: 'Facilitate BMAD kickoff workshop',
            category: 'analysis',
            estimate: 2,
            owner: 'analyst',
            status: 'todo',
            rationale: 'Align on scope, objectives, and risk tolerance.',
        });

        if (risks.length > 0) {
            items.push({
                id: 'bmad-disc-02',
                title: 'Create risk register with mitigations',
                category: 'analysis',
                estimate: 1,
                owner: 'analyst',
                status: 'todo',
                dependencies: risks.map(r => r.id),
                rationale: 'Track and retire risks explicitly.',
            });
        }

        items.push({
            id: 'bmad-disc-03',
            title: 'Capture stakeholder map',
            category: 'analysis',
            estimate: 1,
            owner: 'analyst',
            status: 'todo',
            rationale: `Stakeholders: ${context.stakeholders.join(', ')}`,
        });

        return items;
    }
}
