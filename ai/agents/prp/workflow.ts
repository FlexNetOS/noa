//! PRP Workflow Engine
//!
//! T368: Coordinates signal evaluation and agent recommendations for production readiness.

import { evaluateSignals, signalCount } from './signals';
import { RoboSystemAnalyst } from './system_analyst';
import { RoboAQAAgent } from './aqa';
import { PRPCheck, PRPContext, PRPOutcome, PRPRecommendation } from './types';

export class PRPWorkflowEngine {
    private analyst: RoboSystemAnalyst;
    private aqa: RoboAQAAgent;

    constructor(analyst = new RoboSystemAnalyst(), aqa = new RoboAQAAgent()) {
        this.analyst = analyst;
        this.aqa = aqa;
    }

    async execute(context: PRPContext): Promise<PRPOutcome> {
        const signals = evaluateSignals(context);
        const recommendations = await this.collectRecommendations(context, signals);
        const status = this.computeStatus(signals);
        const notes = [
            `Evaluated ${signals.length} of ${signalCount} signals for ${context.service} (${context.environment}).`,
            `Triggered: ${signals.filter(s => s.triggered).length}.`,
        ];

        return { status, signals, recommendations, notes };
    }

    private async collectRecommendations(context: PRPContext, signals: PRPCheck[]): Promise<PRPRecommendation[]> {
        const analystRecs = this.analyst.diagnose(context, signals);
        const qaRecs = await this.aqa.evaluateQuality(context, signals);
        return [...analystRecs, ...qaRecs];
    }

    private computeStatus(signals: PRPCheck[]): PRPOutcome['status'] {
        if (signals.some(signal => signal.triggered && signal.severity === 'critical')) {
            return 'degraded';
        }
        if (signals.some(signal => signal.triggered && signal.severity === 'warn')) {
            return 'attention';
        }
        return 'healthy';
    }
}
