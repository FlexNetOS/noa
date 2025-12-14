//! ModelSelectorAgent_Technology
//!
//! T476: Implement ModelSelectorAgent_Technology
//! US2: Model selection for technology tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Technology extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Technology', 'technology');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Technology tasks require code understanding and technical reasoning
        const preferredModels = [
            'code-llama-7b',
            'mistral-7b-code',
            'qwen-7b-code',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForTechnology(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for technology task: optimized for code and technical reasoning`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForTechnology(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer code-specialized models
        if (modelId.includes('code')) {
            score += 0.4;
        }

        // Balanced preference
        if (criteria.costPreference === 'balanced') {
            score += 0.2;
        }

        return Math.min(score, 1.0);
    }
}

