//! ModelSelectorAgent_DevOps
//!
//! T467: Implement ModelSelectorAgent_DevOps
//! US2: Model selection for DevOps tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_DevOps extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_DevOps', 'devops');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // DevOps tasks require fast response and infrastructure understanding
        const preferredModels = [
            'mistral-7b',
            'llama-2-7b',
            'qwen-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForDevOps(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for DevOps task: balanced speed and capability`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForDevOps(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer 7B models for speed
        if (modelId.includes('7b')) {
            score += 0.3;
        }

        // Speed is critical for DevOps
        if (criteria.costPreference === 'speed') {
            score += 0.3;
        }

        return Math.min(score, 1.0);
    }
}

