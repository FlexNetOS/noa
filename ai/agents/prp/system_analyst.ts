//! Robo System Analyst
//!
//! T371: Provides system-level diagnostics from PRP signal outcomes.

import { PRPCheck, PRPContext, PRPRecommendation } from './types';

export class RoboSystemAnalyst {
    diagnose(context: PRPContext, signals: PRPCheck[]): PRPRecommendation[] {
        const recommendations: PRPRecommendation[] = [];
        const triggered = signals.filter(signal => signal.triggered);

        triggered.forEach(signal => {
            recommendations.push({
                id: `system-analyst-${signal.id}`,
                title: `Investigate ${signal.id}`,
                owner: 'system-analyst',
                action: this.buildAction(context, signal),
                severity: signal.severity,
                linkedSignals: [signal.id],
            });
        });

        if (triggered.length === 0) {
            recommendations.push({
                id: 'system-analyst-green',
                title: 'System health check',
                owner: 'system-analyst',
                action: 'No blocking signals detected; keep daily PRP checks running.',
                severity: 'info',
                linkedSignals: [],
            });
        }

        return recommendations;
    }

    private buildAction(context: PRPContext, signal: PRPCheck): string {
        if (signal.id === 'latency-slo') {
            return 'Profile hot paths, enable tracing sampling at 10%, and propose caching plan.';
        }
        if (signal.id === 'error-rate-spike') {
            return 'Stop-the-line, enable feature flag gating, and add rollback checkpoint.';
        }
        if (signal.id === 'config-drift' || signal.id === 'schema-drift') {
            return 'Capture current diff, open change request, and lock config drift detector.';
        }
        if (signal.id === 'availability-drop') {
            return 'Review SLOs, validate dependency health, and trigger failover test.';
        }

        return `Add owner to resolve ${signal.id} in ${context.environment} environment.`;
    }
}
