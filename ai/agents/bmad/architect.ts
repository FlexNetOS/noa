//! BMAD Architect Agent
//!
//! T364: Translates analysis into an architecture blueprint and decision log.

import { BMADBacklogItem, BMADBlueprint, BMADContext, BMADRisk } from './types';

export interface ArchitecturePlan {
    blueprint: BMADBlueprint;
    decisions: string[];
    technicalRisks: BMADRisk[];
    backlogUpdates: BMADBacklogItem[];
}

export class BMADArchitectAgent {
    design(context: BMADContext, risks: BMADRisk[], backlog: BMADBacklogItem[]): ArchitecturePlan {
        const blueprint: BMADBlueprint = {
            architectureNotes: this.composeArchitectureNotes(context),
            decisionLog: [],
            nonFunctional: this.defaultNonFunctional(context),
            candidateSolutions: this.defaultCandidateSolutions(context),
        };

        const decisions = this.seedDecisions(context);
        blueprint.decisionLog.push(...decisions);

        const technicalRisks = this.mapRisksToArchitecture(risks);
        const backlogUpdates = this.appendArchitectureTasks(backlog, context);

        return { blueprint, decisions, technicalRisks, backlogUpdates };
    }

    private composeArchitectureNotes(context: BMADContext): string[] {
        return [
            `Primary objective: ${context.objectives[0] || 'Unspecified objective'}`,
            `Constraints: ${context.constraints.length > 0 ? context.constraints.join(', ') : 'None stated'}`,
            `Stakeholders: ${context.stakeholders.join(', ')}`,
        ];
    }

    private defaultNonFunctional(context: BMADContext): string[] {
        const nonFunctional: string[] = [
            'Observability first: tracing + metrics enabled in week 1.',
            'Reliability: target 99.5% availability for MVP services.',
        ];

        if (context.constraints.find(c => c.toLowerCase().includes('pii'))) {
            nonFunctional.push('Privacy: isolate PII handling with access logging and masking.');
        }

        return nonFunctional;
    }

    private defaultCandidateSolutions(context: BMADContext): string[] {
        const stack = context.techStack && context.techStack.length > 0 ? context.techStack.join(', ') : 'tbd';
        return [
            `Leverage existing stack (${stack}) for speed; add adapters only when required.`,
            'Introduce thin orchestration layer so BMAD agents can plug into NOA workflows.',
        ];
    }

    private seedDecisions(context: BMADContext): string[] {
        const decisions: string[] = [];
        decisions.push('Decision: adopt progressive elaboration for requirements, reviewed weekly.');

        if ((context.timelineWeeks || 0) >= 8) {
            decisions.push('Decision: include resilience testing gate before release cut.');
        } else {
            decisions.push('Decision: defer advanced chaos testing to post-MVP increment.');
        }

        return decisions;
    }

    private mapRisksToArchitecture(risks: BMADRisk[]): BMADRisk[] {
        if (risks.length === 0) {
            return [{
                id: 'risk-arch-assumptions',
                description: 'Architecture defined with incomplete risk data; validate assumptions early.',
                impact: 'medium',
                likelihood: 'medium',
                mitigation: 'Add architecture validation spike and update decision log.',
                owner: 'architect',
            }];
        }
        return risks.map(risk => ({
            ...risk,
            owner: risk.owner || 'architect',
        }));
    }

    private appendArchitectureTasks(backlog: BMADBacklogItem[], context: BMADContext): BMADBacklogItem[] {
        const updated = [...backlog];
        updated.push({
            id: 'bmad-arch-01',
            title: 'Draft system context diagram',
            category: 'architecture',
            estimate: 2,
            owner: 'architect',
            status: 'todo',
            rationale: 'Establish shared language for scope and interfaces.',
        });

        updated.push({
            id: 'bmad-arch-02',
            title: 'Select reference architecture and golden paths',
            category: 'architecture',
            estimate: 2,
            owner: 'architect',
            status: 'todo',
            rationale: `Align with ${context.techStack?.join(', ') || 'target stack'}.`,
        });

        return updated;
    }
}
