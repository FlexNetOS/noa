# learning Module

Adaptive learning and model fine-tuning.

**Location**: `sys/core/src/learning/`  
**Feature**: `full`

## Overview

Continuous learning from user interactions:

- Feedback collection
- Preference learning
- Model adaptation
- A/B testing

## Key Types

### LearningLoop

Main learning orchestrator.

```rust
pub struct LearningLoop {
    feedback_store: FeedbackStore,
    adapter: ModelAdapter,
    config: LearningConfig,
}

impl LearningLoop {
    pub async fn record_feedback(&mut self, feedback: Feedback) -> NoaResult<()>;
    pub async fn update_model(&mut self) -> NoaResult<()>;
    pub fn get_metrics(&self) -> LearningMetrics;
}
```

### Feedback

User feedback entry.

```rust
pub struct Feedback {
    pub task_id: TaskId,
    pub rating: Rating,
    pub correction: Option<String>,
    pub timestamp: DateTime<Utc>,
}

pub enum Rating {
    Positive,
    Negative,
    Neutral,
}
```

### LearningMetrics

```rust
pub struct LearningMetrics {
    pub total_feedback: u64,
    pub positive_ratio: f32,
    pub improvement_score: f32,
}
```

## Learning Modes

| Mode | Description |
|------|-------------|
| Passive | Collect feedback only |
| Active | Prompt for feedback |
| Adaptive | Auto-adjust based on patterns |

## Usage

```rust
use noa_core::learning::{LearningLoop, Feedback, Rating};

async fn example(loop_: &mut LearningLoop) -> NoaResult<()> {
    // Record positive feedback
    let feedback = Feedback {
        task_id: "task-123".into(),
        rating: Rating::Positive,
        correction: None,
        timestamp: Utc::now(),
    };
    loop_.record_feedback(feedback).await?;
    
    // Trigger model update
    loop_.update_model().await?;
    
    Ok(())
}
```

## See Also

- [neural module](neural.md) — Model inference
- [memory module](memory.md) — Preference storage
