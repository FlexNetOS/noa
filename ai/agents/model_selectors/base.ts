//! Base ModelSelectorAgent Interface
//!
//! Base class for specialized model selector agents

import { ModelSelectorAgent } from '../../../sys/core/src/agents/model_selector';

export interface ModelSelectionCriteria {
    taskType: string;
    requiredContextLength?: number;
    maxLatencyMs?: number;
    availableResources: {
        maxCpuUsage: number;
        maxMemoryGb: number;
        gpuAvailable: boolean;
        gpuMemoryGb?: number;
    };
    costPreference: 'speed' | 'capability' | 'efficiency' | 'balanced';
}

export interface ModelSelectionResult {
    modelId: string;
    modelName: string;
    confidence: number;
    rationale: string;
}

export abstract class BaseModelSelectorAgent {
    protected agentName: string;
    protected domain: string;

    constructor(agentName: string, domain: string) {
        this.agentName = agentName;
        this.domain = domain;
    }

    abstract selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null>;

    protected getDomainSpecificModels(): string[] {
        // Override in subclasses
        return [];
    }

    protected scoreModel(modelId: string, criteria: ModelSelectionCriteria): number {
        // Base scoring logic
        return 0.5;
    }
}

