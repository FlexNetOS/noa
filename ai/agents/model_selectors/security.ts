//! ModelSelectorAgent_Security
//!
//! T474: Implement ModelSelectorAgent_Security
//! US2: Model selection for security tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Security extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Security', 'security');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Security tasks require high accuracy and reasoning
        const preferredModels = [
            'llama-2-13b',
            'mistral-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForSecurity(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for security task: high accuracy and security reasoning`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForSecurity(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer larger models for accuracy
        if (modelId.includes('13b')) {
            score += 0.4;
        }

        // Capability over speed for security
        if (criteria.costPreference === 'capability') {
            score += 0.3;
        }

        return Math.min(score, 1.0);
    }
}

