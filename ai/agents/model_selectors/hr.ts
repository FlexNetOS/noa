//! ModelSelectorAgent_HR
//!
//! T470: Implement ModelSelectorAgent_HR
//! US2: Model selection for HR tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_HR extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_HR', 'hr');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // HR tasks require good language understanding and empathy
        const preferredModels = [
            'mistral-7b',
            'llama-2-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForHR(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for HR task: good language understanding and balanced performance`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForHR(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Balanced models work well for HR
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

