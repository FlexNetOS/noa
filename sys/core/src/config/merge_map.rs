use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum MergeStrategy {
    /// Insert file under a single top-level key.
    Namespaced,
}

#[derive(Debug, Clone)]
pub struct MergeSpec {
    pub relative_path: &'static str,
    pub raw_key: &'static str,
    pub strategy: MergeStrategy,
}

impl MergeSpec {
    pub fn full_path(&self, noa_root: &Path) -> PathBuf {
        noa_root.join(self.relative_path)
    }
}

pub const CORE_MERGE_SPECS: &[MergeSpec] = &[
    // Layer 1: Base configs (immutable baseline)
    MergeSpec {
        relative_path: "configs/base/ai-providers.json",
        raw_key: "providers",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/shared-resources.json",
        raw_key: "shared_resources",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/noa-server.json",
        raw_key: "noa_server",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/database.yaml",
        raw_key: "database_config",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/observability.yaml",
        raw_key: "observability",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/minio.yaml",
        raw_key: "minio",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/qdrant.yaml",
        raw_key: "qdrant",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/quickwit.yaml",
        raw_key: "quickwit",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/tools.json",
        raw_key: "tools",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/bootstrap-tools.json",
        raw_key: "bootstrap_tools",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/base/bootstrap-state.json",
        raw_key: "bootstrap_state",
        strategy: MergeStrategy::Namespaced,
    },

    // Layer 2: Semantic configs (mutable preferences)
    MergeSpec {
        relative_path: "configs/semantic/device-orchestration.json",
        raw_key: "device_orchestration",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/semantic/desktop-apps.json",
        raw_key: "desktop_apps",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/semantic/agent-rules/git-conflict-ai.json",
        raw_key: "git_conflict_ai",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/semantic/agent-rules/git-local-cicd.json",
        raw_key: "git_local_cicd",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "configs/semantic/agent-rules/git-pr-workflow.json",
        raw_key: "git_pr_workflow",
        strategy: MergeStrategy::Namespaced,
    },

    // Layer 3: Enforcement configs (policy engine)
    MergeSpec {
        relative_path: "configs/enforcement/policy-engine/kernel-selection-policy.json",
        raw_key: "kernel_selection_policy",
        strategy: MergeStrategy::Namespaced,
    },
];
