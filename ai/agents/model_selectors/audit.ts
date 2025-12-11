//! ModelSelectorAgent_Audit
//!
//! T465: Implement ModelSelectorAgent_Audit
//! US2: Model selection for audit tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_Audit extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_Audit', 'audit');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Audit tasks require high accuracy and reasoning capability
        const preferredModels = [
            'llama-2-13b',
            'mistral-7b',
            'qwen-7b',
        ];

        // Prefer models with larger context for audit logs
        const contextRequirement = criteria.requiredContextLength || 4096;

        // Select best model based on criteria
        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForAudit(modelId, criteria, contextRequirement);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for audit task: high accuracy and ${contextRequirement} context required`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForAudit(
        modelId: string,
        criteria: ModelSelectionCriteria,
        contextRequirement: number
    ): number {
        let score = 0.5;

        // Prefer larger models for audit accuracy
        if (modelId.includes('13b') || modelId.includes('70b')) {
            score += 0.3;
        } else if (modelId.includes('7b')) {
            score += 0.2;
        }

        // Prefer models with sufficient context
        if (contextRequirement <= 4096) {
            score += 0.2;
        }

        // Cost preference
        if (criteria.costPreference === 'capability') {
            score += 0.2;
        } else if (criteria.costPreference === 'efficiency') {
            score -= 0.1;
        }

        return Math.min(score, 1.0);
    }
}

