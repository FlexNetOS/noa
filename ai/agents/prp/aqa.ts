//! Robo AQA Agent
//!
//! T372: Automated quality analyst focused on test, coverage, and release fitness.

import { PRPCheck, PRPContext, PRPRecommendation } from './types';

export class RoboAQAAgent {
    async evaluateQuality(context: PRPContext, signals: PRPCheck[]): Promise<PRPRecommendation[]> {
        const qualitySignals = signals.filter(signal => this.isQualitySignal(signal.id));
        const recommendations: PRPRecommendation[] = qualitySignals
            .filter(signal => signal.triggered)
            .map(signal => this.toRecommendation(signal));

        if (recommendations.length === 0) {
            recommendations.push({
                id: 'aqa-green',
                title: 'QA posture healthy',
                owner: 'qa',
                action: 'Maintain automation suite and keep PRP loop-mode running nightly.',
                severity: 'info',
                linkedSignals: [],
            });
        }

        if ((context.metrics.pipelineFlakeRate ?? 0) > 0.02) {
            recommendations.push({
                id: 'aqa-flake-burn-down',
                title: 'Stabilize flaky pipelines',
                owner: 'qa',
                action: 'Quarantine flaky specs, capture failure signatures, and add rerun budget caps.',
                severity: 'warn',
                linkedSignals: ['pipeline-flake'],
            });
        }

        return recommendations;
    }

    private isQualitySignal(id: string): boolean {
        return [
            'coverage-low',
            'pipeline-flake',
            'change-failure',
            'deploy-frequency-low',
            'pr-cycle-slow',
            'error-rate-spike',
            'rollback-frequency',
        ].includes(id);
    }

    private toRecommendation(signal: PRPCheck): PRPRecommendation {
        const actionMap: Record<string, string> = {
            'coverage-low': 'Lift coverage to 80% by adding regression tests to critical paths.',
            'pipeline-flake': 'Isolate flaky tests, add retry budget, and track flake IDs in CI.',
            'change-failure': 'Add canary + smoke suites before promotion.',
            'deploy-frequency-low': 'Automate deploy pipeline with feature flags for safe rollout.',
            'pr-cycle-slow': 'Enforce smaller PRs and enable parallel reviews.',
            'error-rate-spike': 'Block new deploys until error rate is back under 1%.',
            'rollback-frequency': 'Perform root-cause session and add pre-deploy gates.',
        };

        return {
            id: `aqa-${signal.id}`,
            title: `QA action: ${signal.id}`,
            owner: 'qa',
            action: actionMap[signal.id] || `Address quality risk for ${signal.id}`,
            severity: signal.severity,
            linkedSignals: [signal.id],
        };
    }
}
