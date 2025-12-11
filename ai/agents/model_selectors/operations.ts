//! ModelSelectorAgent_Operations
//!
//! T473: Implement ModelSelectorAgent_Operations
//! US2: Model selection for operations tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Operations extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Operations', 'operations');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Operations tasks require fast response and reliability
        const preferredModels = [
            'mistral-7b',
            'llama-2-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForOperations(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for operations task: fast response and reliability`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForOperations(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer 7B models for speed
        if (modelId.includes('7b')) {
            score += 0.3;
        }

        // Speed is critical
        if (criteria.costPreference === 'speed') {
            score += 0.3;
        }

        return Math.min(score, 1.0);
    }
}

