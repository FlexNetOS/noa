//! BMAD Product Owner Agent
//!
//! T365: Prioritizes discovery/architecture backlog and keeps a lightweight release plan.

import { BMADBacklogItem, BMADRecommendation, Priority } from './types';

export interface PrioritizedBacklog {
    items: BMADBacklogItem[];
    releasePlan: string[];
}

export class BMADProductOwnerAgent {
    prioritize(backlog: BMADBacklogItem[], recommendations: BMADRecommendation[]): PrioritizedBacklog {
        const items = backlog.map(item => ({ ...item }));
        const priorities = this.derivePriorities(items, recommendations);
        const ordered = items.sort((a, b) => priorities.get(a.id)! - priorities.get(b.id)!);

        const releasePlan = ordered.map(item => `${item.id}: ${item.title}`);

        return { items: ordered, releasePlan };
    }

    private derivePriorities(backlog: BMADBacklogItem[], recommendations: BMADRecommendation[]): Map<string, number> {
        const map = new Map<string, number>();

        const priorityWeight: Record<Priority, number> = { p0: 0, p1: 1, p2: 2 };
        const recLookup = new Map<string, Priority>();

        recommendations.forEach((rec, idx) => {
            recLookup.set(rec.summary.toLowerCase(), rec.priority);
            // Ensure deterministic ordering
            recLookup.set(`${rec.summary.toLowerCase()}-${idx}`, rec.priority);
        });

        backlog.forEach((item, idx) => {
            let weight = 2; // default priority
            const summaryMatch = recLookup.get(item.title.toLowerCase());
            if (summaryMatch) {
                weight = priorityWeight[summaryMatch];
            }
            if (item.category === 'analysis') {
                weight = Math.min(weight, 0);
            }
            if (item.category === 'architecture') {
                weight = Math.min(weight, 1);
            }
            // tie-breaker to keep stable order
            weight += idx * 0.01;
            map.set(item.id, weight);
        });

        return map;
    }
}
