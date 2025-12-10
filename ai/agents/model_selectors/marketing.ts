//! ModelSelectorAgent_Marketing
//!
//! T472: Implement ModelSelectorAgent_Marketing
//! US2: Model selection for marketing tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Marketing extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Marketing', 'marketing');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Marketing tasks require creativity and language generation
        const preferredModels = [
            'mistral-7b',
            'llama-2-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForMarketing(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for marketing task: good language generation and creativity`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForMarketing(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Balanced models for creativity
        if (modelId.includes('7b')) {
            score += 0.3;
        }

        // Balanced preference
        if (criteria.costPreference === 'balanced') {
            score += 0.2;
        }

        return Math.min(score, 1.0);
    }
}

