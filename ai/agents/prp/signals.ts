//! PRP Signal System
//!
//! T369: Implements 30+ production readiness signals the workflow engine can evaluate.

import { PRPCheck, PRPContext, Severity } from './types';

interface PRPSignal {
    id: string;
    description: string;
    severity: Severity;
    detector: (context: PRPContext) => PRPCheck;
}

type Comparator = 'gt' | 'lt';

const thresholdSignal = (
    id: string,
    description: string,
    severity: Severity,
    selector: (context: PRPContext) => number,
    limit: number,
    comparator: Comparator,
): PRPSignal => ({
    id,
    description,
    severity,
    detector: (context: PRPContext): PRPCheck => {
        const value = selector(context);
        const triggered = comparator === 'gt' ? value > limit : value < limit;
        const reason = triggered
            ? `${description} (value=${value}, limit=${limit})`
            : 'within threshold';
        return { id, triggered, reason, severity };
    },
});

const SIGNALS: PRPSignal[] = [
    thresholdSignal('latency-slo', 'p95 latency above target', 'critical', ctx => ctx.metrics.latencyP95Ms ?? 0, 500, 'gt'),
    thresholdSignal('error-rate-spike', 'error rate above 1%', 'critical', ctx => ctx.metrics.errorRate ?? 0, 0.01, 'gt'),
    thresholdSignal('availability-drop', 'availability below 99%', 'critical', ctx => ctx.metrics.availability ?? 1, 0.99, 'lt'),
    thresholdSignal('throughput-drop', 'throughput drop detected', 'warn', ctx => ctx.metrics.throughputRps ?? 0, 50, 'lt'),
    thresholdSignal('backlog-overflow', 'backlog exceeds safe threshold', 'warn', ctx => ctx.metrics.backlogSize ?? 0, 120, 'gt'),
    thresholdSignal('coverage-low', 'test coverage below 80%', 'warn', ctx => ctx.metrics.testCoverage ?? 1, 0.8, 'lt'),
    thresholdSignal('defects-open', 'open defects exceed target', 'warn', ctx => ctx.metrics.openDefects ?? 0, 15, 'gt'),
    thresholdSignal('mttr-high', 'MTTR above 12h', 'critical', ctx => ctx.metrics.mttrHours ?? 0, 12, 'gt'),
    thresholdSignal('change-failure', 'change failure rate high', 'warn', ctx => ctx.metrics.changeFailureRate ?? 0, 0.15, 'gt'),
    thresholdSignal('deploy-frequency-low', 'deploy frequency below daily', 'warn', ctx => ctx.metrics.deployFrequencyPerDay ?? 0, 1, 'lt'),
    thresholdSignal('incidents-spike', 'incidents in last 24h > 2', 'warn', ctx => ctx.metrics.incidents24h ?? 0, 2, 'gt'),
    thresholdSignal('pager-fatigue', 'pager load high', 'warn', ctx => ctx.metrics.pagerLoad ?? 0, 5, 'gt'),
    thresholdSignal('config-drift', 'configuration drift detected', 'warn', ctx => ctx.metrics.configDriftScore ?? 0, 1, 'gt'),
    thresholdSignal('schema-drift', 'database schema drift detected', 'warn', ctx => ctx.metrics.schemaDriftScore ?? 0, 1, 'gt'),
    thresholdSignal('vuln-blocker', 'unpatched vulnerabilities present', 'critical', ctx => ctx.metrics.vulnFindings ?? 0, 0, 'gt'),
    thresholdSignal('secrets-incident', 'secrets handling incident risk', 'critical', ctx => ctx.metrics.secretsIncidents ?? 0, 0, 'gt'),
    thresholdSignal('backlog-churn', 'backlog churn exceeds 25%', 'warn', ctx => ctx.metrics.backlogChurn ?? 0, 0.25, 'gt'),
    thresholdSignal('pr-cycle-slow', 'PR cycle time above 24h', 'warn', ctx => ctx.metrics.prCycleTimeHours ?? 0, 24, 'gt'),
    thresholdSignal('lead-time-slow', 'lead time above 7d', 'warn', ctx => ctx.metrics.leadTimeDays ?? 0, 7, 'gt'),
    thresholdSignal('error-budget-burn', 'error budget burn above 20%', 'critical', ctx => ctx.metrics.errorBudgetBurn ?? 0, 0.2, 'gt'),
    thresholdSignal('cpu-saturation', 'CPU saturation above 70%', 'warn', ctx => ctx.metrics.cpuSaturation ?? 0, 0.7, 'gt'),
    thresholdSignal('data-freshness', 'data freshness exceeds 4h', 'warn', ctx => ctx.metrics.dataFreshnessHours ?? 0, 4, 'gt'),
    thresholdSignal('pipeline-flake', 'pipeline flake rate above 5%', 'warn', ctx => ctx.metrics.pipelineFlakeRate ?? 0, 0.05, 'gt'),
    thresholdSignal('rollback-frequency', 'recent rollback detected', 'warn', ctx => ctx.metrics.rollbacks ?? 0, 0, 'gt'),
    thresholdSignal('p0-open', 'P0 incidents still open', 'critical', ctx => ctx.metrics.p0Open ?? 0, 0, 'gt'),
    thresholdSignal('doc-gap', 'documentation coverage below 70%', 'warn', ctx => ctx.metrics.docCoverage ?? 1, 0.7, 'lt'),
    thresholdSignal('runtime-drift', 'runtime drift detected', 'warn', ctx => ctx.metrics.runtimeDrift ?? 0, 0.1, 'gt'),
    thresholdSignal('qa-automation-low', 'QA automation below 60%', 'warn', ctx => ctx.metrics.qaAutomationRate ?? 1, 0.6, 'lt'),
    thresholdSignal('ux-debt', 'UX debt index high', 'warn', ctx => ctx.metrics.uxDebtIndex ?? 0, 0.6, 'gt'),
    thresholdSignal('security-debt', 'Security debt index high', 'critical', ctx => ctx.metrics.securityDebtIndex ?? 0, 0.5, 'gt'),
    thresholdSignal('observability-gap', 'Observability coverage below 75%', 'warn', ctx => ctx.metrics.observabilityCoverage ?? 1, 0.75, 'lt'),
    thresholdSignal('ai-policy-gap', 'AI policy gaps detected', 'warn', ctx => ctx.metrics.aiPolicyGaps ?? 0, 0, 'gt'),
];

export const evaluateSignals = (context: PRPContext): PRPCheck[] => SIGNALS.map(signal => signal.detector(context));

export const signalCount = SIGNALS.length;
