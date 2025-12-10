//! ModelSelectorAgent_DataStack
//!
//! T466: Implement ModelSelectorAgent_DataStack
//! US2: Model selection for data stack tasks

import { BaseModelSelectorAgent, ModelSelectionCriteria, ModelSelectionResult } from './base';

export class ModelSelectorAgent_DataStack extends BaseModelSelectorAgent {
    constructor() {
        super('ModelSelectorAgent_DataStack', 'data_stack');
    }

    async selectModel(criteria: ModelSelectionCriteria): Promise<ModelSelectionResult | null> {
        // Data stack tasks require good code generation and SQL understanding
        const preferredModels = [
            'code-llama-7b',
            'mistral-7b-code',
            'qwen-7b-code',
        ];

        let bestModel: ModelSelectionResult | null = null;
        let bestScore = 0;

        for (const modelId of preferredModels) {
            const score = this.scoreModelForDataStack(modelId, criteria);
            if (score > bestScore) {
                bestScore = score;
                bestModel = {
                    modelId,
                    modelName: modelId,
                    confidence: score,
                    rationale: `Selected ${modelId} for data stack task: optimized for code and SQL generation`,
                };
            }
        }

        return bestModel;
    }

    private scoreModelForDataStack(modelId: string, criteria: ModelSelectionCriteria): number {
        let score = 0.5;

        // Prefer code-specialized models
        if (modelId.includes('code')) {
            score += 0.4;
        }

        // Speed preference for data operations
        if (criteria.costPreference === 'speed') {
            score += 0.2;
        }

        return Math.min(score, 1.0);
    }
}

