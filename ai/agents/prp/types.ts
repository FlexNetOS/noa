//! PRP Shared Types
//!
//! Provides lightweight contracts for PRP workflow components.

export type Severity = 'info' | 'warn' | 'critical';

export interface PRPContext {
    service: string;
    environment: string;
    objectives: string[];
    metrics: {
        latencyP95Ms?: number;
        errorRate?: number;
        availability?: number;
        throughputRps?: number;
        backlogSize?: number;
        testCoverage?: number;
        openDefects?: number;
        mttrHours?: number;
        changeFailureRate?: number;
        deployFrequencyPerDay?: number;
        incidents24h?: number;
        pagerLoad?: number;
        configDriftScore?: number;
        schemaDriftScore?: number;
        vulnFindings?: number;
        secretsIncidents?: number;
        backlogChurn?: number;
        prCycleTimeHours?: number;
        leadTimeDays?: number;
        errorBudgetBurn?: number;
        cpuSaturation?: number;
        dataFreshnessHours?: number;
        pipelineFlakeRate?: number;
        rollbacks?: number;
        p0Open?: number;
        docCoverage?: number;
        runtimeDrift?: number;
        qaAutomationRate?: number;
        uxDebtIndex?: number;
        securityDebtIndex?: number;
        observabilityCoverage?: number;
        aiPolicyGaps?: number;
    };
    annotations?: string[];
}

export interface PRPCheck {
    id: string;
    triggered: boolean;
    reason: string;
    severity: Severity;
}

export interface PRPRecommendation {
    id: string;
    title: string;
    owner: string;
    action: string;
    severity: Severity;
    linkedSignals: string[];
}

export interface PRPOutcome {
    status: 'healthy' | 'attention' | 'degraded';
    signals: PRPCheck[];
    recommendations: PRPRecommendation[];
    notes: string[];
}

export interface PRPLoopIteration {
    iteration: number;
    outcome: PRPOutcome;
    timestamp: string;
}
