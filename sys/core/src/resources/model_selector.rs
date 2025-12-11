//! Model size selector based on hardware tier
//!
//! T737: Implement model size selection per hardware.

use crate::platform::capabilities::{CapabilitySnapshot, HardwareTier};
use serde::{Deserialize, Serialize};

/// Model size tiers used for selection heuristics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelSizeClass {
    Tiny,
    Small,
    Medium,
    Large,
}

/// Recommended model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSelection {
    pub size: ModelSizeClass,
    pub preferred_quantization: String,
}

/// Selector that maps capability snapshot to a model size.
pub struct ModelSelector;

impl ModelSelector {
    pub fn select(snapshot: &CapabilitySnapshot) -> ModelSelection {
        match snapshot.tier {
            HardwareTier::High => ModelSelection {
                size: ModelSizeClass::Large,
                preferred_quantization: "f16".to_string(),
            },
            HardwareTier::Medium => ModelSelection {
                size: ModelSizeClass::Medium,
                preferred_quantization: "q5_0".to_string(),
            },
            HardwareTier::Low => ModelSelection {
                size: ModelSizeClass::Small,
                preferred_quantization: "q4_0".to_string(),
            },
            HardwareTier::Unknown => ModelSelection {
                size: ModelSizeClass::Tiny,
                preferred_quantization: "q4_0".to_string(),
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
                cores: 4,
                threads: 8,
            },
            memory_bytes: 8 * 1024 * 1024 * 1024,
            gpus: vec![],
            tier,
        }
    }

    #[test]
    fn chooses_model_sizes() {
        let medium = ModelSelector::select(&snapshot(HardwareTier::Medium));
        assert_eq!(medium.size, ModelSizeClass::Medium);

        let low = ModelSelector::select(&snapshot(HardwareTier::Low));
        assert_eq!(low.preferred_quantization, "q4_0");
    }
}
