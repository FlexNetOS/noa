//! PRP Loop Mode
//!
//! T370: Runs PRP in a repeating loop to validate fixes and trend improvements.

import { PRPContext, PRPLoopIteration } from './types';
import { PRPWorkflowEngine } from './workflow';

export class PRPLoopMode {
    async run(engine: PRPWorkflowEngine, context: PRPContext, iterations = 3): Promise<PRPLoopIteration[]> {
        const history: PRPLoopIteration[] = [];
        let workingContext: PRPContext = { ...context, metrics: { ...context.metrics } };

        for (let i = 0; i < iterations; i++) {
            const outcome = await engine.execute(workingContext);
            history.push({
                iteration: i + 1,
                outcome,
                timestamp: new Date().toISOString(),
            });

            // Simple adaptive tweak: assume each iteration reduces a subset of risks
            workingContext = this.applyAdaptiveTweaks(workingContext, outcome);
        }

        return history;
    }

    private applyAdaptiveTweaks(context: PRPContext, outcome: PRPLoopIteration['outcome']): PRPContext {
        const next = { ...context, metrics: { ...context.metrics } };
        const triggered = outcome.signals.filter(signal => signal.triggered);

        // Reduce severity for common metrics to simulate remediation
        if (triggered.find(s => s.id === 'latency-slo')) {
            next.metrics.latencyP95Ms = Math.max(0, (next.metrics.latencyP95Ms ?? 0) - 50);
        }
        if (triggered.find(s => s.id === 'error-rate-spike')) {
            next.metrics.errorRate = Math.max(0, (next.metrics.errorRate ?? 0) - 0.002);
        }
        if (triggered.find(s => s.id === 'coverage-low')) {
            next.metrics.testCoverage = Math.min(1, (next.metrics.testCoverage ?? 0.7) + 0.02);
        }

        return next;
    }
}
