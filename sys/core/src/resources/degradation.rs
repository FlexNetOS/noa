//! Graceful degradation strategies
//!
//! T738: Implement graceful degradation on low-resource systems.

use crate::platform::capabilities::{CapabilitySnapshot, HardwareTier};
use crate::resources::model_selector::{ModelSelection, ModelSelector, ModelSizeClass};
use serde::{Deserialize, Serialize};

/// Guidance for stepping down resource usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationPlan {
    pub reduce_parallelism: bool,
    pub allow_gpu: bool,
    pub fallback_model: Option<ModelSelection>,
    pub notes: Vec<String>,
}

/// Advisor that produces degradation plans.
pub struct DegradationAdvisor;

impl DegradationAdvisor {
    pub fn for_snapshot(snapshot: &CapabilitySnapshot) -> DegradationPlan {
        match snapshot.tier {
            HardwareTier::High => DegradationPlan {
                reduce_parallelism: false,
                allow_gpu: true,
                fallback_model: None,
                notes: vec!["High tier: full capabilities enabled".to_string()],
            },
            HardwareTier::Medium => DegradationPlan {
                reduce_parallelism: false,
                allow_gpu: !snapshot.gpus.is_empty(),
                fallback_model: Some(ModelSelector::select(snapshot)),
                notes: vec!["Medium tier: keep GPU optional, limit burst".to_string()],
            },
            HardwareTier::Low => DegradationPlan {
                reduce_parallelism: true,
                allow_gpu: false,
                fallback_model: Some(ModelSelection {
                    size: ModelSizeClass::Small,
                    preferred_quantization: "q4_0",
                }),
                notes: vec!["Low tier: reduce concurrency and use small model".to_string()],
            },
            HardwareTier::Unknown => DegradationPlan {
                reduce_parallelism: true,
                allow_gpu: false,
                fallback_model: Some(ModelSelection {
                    size: ModelSizeClass::Tiny,
                    preferred_quantization: "q4_0",
                }),
                notes: vec!["Unknown tier: enable defensive defaults".to_string()],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural::hardware::CpuInfo;
    use crate::platform::capabilities::CapabilitySnapshot;

    fn snapshot(tier: HardwareTier) -> CapabilitySnapshot {
        CapabilitySnapshot {
            platform: crate::platform::detect::platform_info(),
            cpu: CpuInfo {
                cores: 2,
                threads: 4,
            },
            memory_bytes: 4 * 1024 * 1024 * 1024,
            gpus: vec![],
            tier,
        }
    }

    #[test]
    fn degrades_low_tier() {
        let plan = DegradationAdvisor::for_snapshot(&snapshot(HardwareTier::Low));
        assert!(plan.reduce_parallelism);
        assert!(!plan.allow_gpu);
        assert!(plan.fallback_model.is_some());
    }
}
