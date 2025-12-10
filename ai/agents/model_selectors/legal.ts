//! ModelSelectorAgent_LegalCompliance
//!
//! T471: Implement ModelSelectorAgent_LegalCompliance
//! US2: Model selection for legal/compliance tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_LegalCompliance extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_LegalCompliance', 'legal');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Legal tasks require high accuracy and large context
        const preferredModels = [
            'llama-2-13b',
            'mistral-7b',
            'qwen-7b',
        ];

        const contextRequirement = criteria.requiredContextLength || 8192;

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForLegal(modelId, criteria, contextRequirement);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for legal task: high accuracy and ${contextRequirement} context for document analysis`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForLegal(
        modelId: string,
        criteria: ModelSelectionCriteria,
        contextRequirement: number
    ): number {
        let score = 0.5;

        // Prefer larger models for accuracy
        if (modelId.includes('13b')) {
            score += 0.4;
        }

        // Large context is critical
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

