//! BMAD Shared Types
//! 
//! Defines the common data contracts used by BMAD agents to keep orchestration
//! predictable and auditable across the analyst, architect, PO, and dev roles.

export type RiskLevel = 'low' | 'medium' | 'high';
export type Priority = 'p0' | 'p1' | 'p2';

export interface BMADContext {
    initiative: string;
    objectives: string[];
    constraints: string[];
    stakeholders: string[];
    timelineWeeks?: number;
    riskTolerance?: RiskLevel;
    techStack?: string[];
    dependencies?: string[];
}

export interface BMADRisk {
    id: string;
    description: string;
    impact: RiskLevel;
    likelihood: RiskLevel;
    mitigation: string;
    owner?: string;
}

export interface BMADRecommendation {
    summary: string;
    details: string[];
    owner?: string;
    priority: Priority;
}

export interface BMADBacklogItem {
    id: string;
    title: string;
    category: 'analysis' | 'architecture' | 'delivery';
    estimate?: number;
    owner?: string;
    status?: 'todo' | 'doing' | 'done';
    rationale?: string;
    dependencies?: string[];
}

export interface BMADBlueprint {
    architectureNotes: string[];
    decisionLog: string[];
    nonFunctional: string[];
    candidateSolutions: string[];
}

export interface BMADWorkflowSnapshot {
    findings: {
        risks: BMADRisk[];
        recommendations: BMADRecommendation[];
    };
    backlog: BMADBacklogItem[];
    blueprint: BMADBlueprint;
    artifacts: string[];
}
