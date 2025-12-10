//! Dynamic resource allocator
//!
//! T736: Implement dynamic resource allocation based on hardware tier.

use crate::platform::capabilities::{CapabilitySnapshot, HardwareTier};
use crate::resources::degradation::DegradationPlan;
use crate::resources::model_selector::{ModelSelection, ModelSelector};
use serde::{Deserialize, Serialize};

/// Recommended allocation for a given hardware snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationPlan {
    pub max_concurrent_tasks: usize,
    pub max_inference_batch: usize,
    pub use_gpu: bool,
    pub model: ModelSelection,
    pub notes: Vec<String>,
}

/// Allocator that maps hardware capability into runtime limits.
pub struct ResourceAllocator;

impl ResourceAllocator {
    pub fn plan(snapshot: &CapabilitySnapshot) -> ResourceAllocationPlan {
        let model = ModelSelector::select(snapshot);
        let (max_concurrent_tasks, max_inference_batch, use_gpu) = match snapshot.tier {
            HardwareTier::High => (12, 16, true),
            HardwareTier::Medium => (6, 8, !snapshot.gpus.is_empty()),
            HardwareTier::Low => (3, 4, false),
            HardwareTier::Unknown => (2, 2, false),
        };

        ResourceAllocationPlan {
            max_concurrent_tasks,
            max_inference_batch,
            use_gpu,
            model,
            notes: vec![format!(
                "Tier {:?} with {} threads, {} GPU(s)",
                snapshot.tier,
                snapshot.cpu.threads,
                snapshot.gpus.len()
            )],
        }
    }

    /// Apply graceful degradation hints to an existing plan.
    pub fn apply_degradation(
        plan: &ResourceAllocationPlan,
        degradation: &DegradationPlan,
    ) -> ResourceAllocationPlan {
        let mut adjusted = plan.clone();
        if degradation.reduce_parallelism {
            adjusted.max_concurrent_tasks = adjusted.max_concurrent_tasks.saturating_sub(2).max(1);
            adjusted.max_inference_batch = adjusted.max_inference_batch.saturating_sub(2).max(1);
        }
        if !degradation.allow_gpu {
            adjusted.use_gpu = false;
        }
        adjusted.model = degradation.fallback_model.unwrap_or(plan.model);
        adjusted.notes.extend(degradation.notes.clone());
        adjusted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural::hardware::CpuInfo;
    use crate::platform::capabilities::{CapabilitySnapshot, HardwareTier};

    fn snapshot(tier: HardwareTier) -> CapabilitySnapshot {
        CapabilitySnapshot {
            platform: crate::platform::detect::platform_info(),
            cpu: CpuInfo { cores: 4, threads: 8 },
            memory_bytes: 16 * 1024 * 1024 * 1024,
            gpus: vec![],
            tier,
        }
    }

    #[test]
    fn builds_plan_for_tier() {
        let plan = ResourceAllocator::plan(&snapshot(HardwareTier::Medium));
        assert!(plan.max_concurrent_tasks > 0);
        assert!(plan.max_inference_batch > 0);
    }

    #[test]
    fn applies_degradation() {
        let plan = ResourceAllocator::plan(&snapshot(HardwareTier::High));
        let degradation = DegradationPlan {
            reduce_parallelism: true,
            allow_gpu: false,
            fallback_model: None,
            notes: vec!["memory pressure detected".to_string()],
        };
        let adjusted = ResourceAllocator::apply_degradation(&plan, &degradation);
        assert!(adjusted.max_concurrent_tasks <= plan.max_concurrent_tasks);
        assert!(!adjusted.use_gpu);
        assert!(adjusted.notes.iter().any(|n| n.contains("memory")));
    }
}
