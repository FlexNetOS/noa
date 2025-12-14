//! ModelSelectorAgent_Strategy
//!
//! T475: Implement ModelSelectorAgent_Strategy
//! US2: Model selection for strategy tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Strategy extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Strategy', 'strategy');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Strategy tasks require high reasoning and long context
        const preferredModels = [
            'llama-2-13b',
            'mistral-7b',
            'qwen-7b',
        ];

        const contextRequirement = criteria.requiredContextLength || 8192;

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForStrategy(modelId, criteria, contextRequirement);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for strategy task: high reasoning and ${contextRequirement} context for strategic analysis`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForStrategy(
        modelId: string,
        criteria: ModelSelectionCriteria,
        contextRequirement: number
    ): number {
        let score = 0.5;

        // Prefer larger models for reasoning
        if (modelId.includes('13b')) {
            score += 0.4;
        }

        // Large context for strategic documents
        if (contextRequirement >= 8192) {
            score += 0.3;
        }

        // Capability over speed
        if (criteria.costPreference === 'capability') {
            score += 0.2;
        }

        return Math.min(score, 1.0);
    }
}

