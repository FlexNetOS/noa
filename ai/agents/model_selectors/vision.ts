//! ModelSelectorAgent_Vision
//!
//! T477: Implement ModelSelectorAgent_Vision
//! US2: Model selection for vision tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Vision extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Vision', 'vision');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Vision tasks require vision-capable models
        const preferredModels = [
            'llava-7b',
            'llava-13b',
            'qwen-vl-7b',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForVision(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for vision task: vision-capable model for image understanding`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForVision(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer vision models
        if (modelId.includes('llava') || modelId.includes('vl')) {
            score += 0.5;
        }

        // Larger models for better vision understanding
        if (modelId.includes('13b')) {
            score += 0.2;
        }

        // Capability for vision tasks
        if (criteria.costPreference === 'capability') {
            score += 0.2;
        }

        return Math.min(score, 1.0);
    }
}

