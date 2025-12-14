//! ModelSelectorAgent_Finance
//!
//! T469: Implement ModelSelectorAgent_Finance
//! US2: Model selection for finance tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Finance extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Finance', 'finance');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Finance tasks require high accuracy and numerical reasoning
        const preferredModels = [
            'llama-2-13b',
            'mistral-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForFinance(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for finance task: high accuracy and numerical reasoning`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForFinance(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer larger models for accuracy
        if (modelId.includes('13b')) {
            score += 0.4;
        }

        // Accuracy over speed for finance
        if (criteria.costPreference === 'capability') {
            score += 0.3;
        }

        return Math.min(score, 1.0);
    }
}

