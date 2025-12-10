//! BMAD Dev Agent
//!
//! T366: Translates the PO backlog and architecture into implementable slices.

import { BMADBacklogItem, BMADBlueprint } from './types';

export interface ImplementationPlan {
    backlog: BMADBacklogItem[];
    artifacts: string[];
    handoffNotes: string[];
}

export class BMADDevAgent {
    prepareImplementation(blueprint: BMADBlueprint, backlog: BMADBacklogItem[]): ImplementationPlan {
        const deliveryBacklog = this.expandDeliveryBacklog(backlog);
        const artifacts = this.planArtifacts(blueprint);
        const handoffNotes = this.createHandoffNotes(deliveryBacklog);

        return { backlog: deliveryBacklog, artifacts, handoffNotes };
    }

    private expandDeliveryBacklog(backlog: BMADBacklogItem[]): BMADBacklogItem[] {
        const delivery: BMADBacklogItem[] = [
            {
                id: 'bmad-dev-01',
                title: 'Set up CI checks for BMAD artifacts',
                category: 'delivery',
                estimate: 2,
                owner: 'dev',
                status: 'todo',
                rationale: 'Guarantee repeatable BMAD runs and auditability.',
            },
            {
                id: 'bmad-dev-02',
                title: 'Automate BMAD report generation',
                category: 'delivery',
                estimate: 3,
                owner: 'dev',
                status: 'todo',
                rationale: 'Produce consistent BMAD outputs for stakeholders.',
            },
        ];

        return [...backlog, ...delivery];
    }

    private planArtifacts(blueprint: BMADBlueprint): string[] {
        const artifacts = [
            'BMAD_SNAPSHOT.md',
            'DECISION_LOG.md',
            'RISK_REGISTER.md',
        ];

        if (blueprint.candidateSolutions.length > 0) {
            artifacts.push('ARCH_OPTIONS.md');
        }

        return artifacts;
    }

    private createHandoffNotes(backlog: BMADBacklogItem[]): string[] {
        const highImpact = backlog.filter(item => item.category !== 'analysis').map(item => item.id);
        return [
            'Ensure PO sign-off on backlog ordering before sprint start.',
            `Track completion of high-impact items: ${highImpact.join(', ') || 'none'}.`,
        ];
    }
}
