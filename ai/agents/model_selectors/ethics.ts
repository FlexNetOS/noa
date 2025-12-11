//! ModelSelectorAgent_Ethics
//!
//! T468: Implement ModelSelectorAgent_Ethics
//! US2: Model selection for ethics tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Ethics extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Ethics', 'ethics');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Ethics tasks require high reasoning capability and safety
        const preferredModels = [
            'llama-2-13b',
            'mistral-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForEthics(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for ethics task: high reasoning and safety considerations`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForEthics(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer larger models for better reasoning
        if (modelId.includes('13b')) {
            score += 0.4;
        }

        // Capability over speed for ethics
        if (criteria.costPreference === 'capability') {
            score += 0.3;
        }

        return Math.min(score, 1.0);
    }
}

