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
    // Existing merges
    MergeSpec {
        relative_path: "config/ai-providers.json",
        raw_key: "providers",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/shared-resources.json",
        raw_key: "shared_resources",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/noa-server.json",
        raw_key: "noa_server",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/device-orchestration.json",
        raw_key: "device_orchestration",
        strategy: MergeStrategy::Namespaced,
    },

    // New: service configs
    MergeSpec {
        relative_path: "config/database.yaml",
        raw_key: "database_config",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/observability.yaml",
        raw_key: "observability",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/minio.yaml",
        raw_key: "minio",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/qdrant.yaml",
        raw_key: "qdrant",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/quickwit.yaml",
        raw_key: "quickwit",
        strategy: MergeStrategy::Namespaced,
    },

    // New: policy / workflow configs
    MergeSpec {
        relative_path: "config/kernel-selection-policy.json",
        raw_key: "kernel_selection_policy",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/desktop-apps.json",
        raw_key: "desktop_apps",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/git-conflict-ai.json",
        raw_key: "git_conflict_ai",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/git-local-cicd.json",
        raw_key: "git_local_cicd",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/git-pr-workflow.json",
        raw_key: "git_pr_workflow",
        strategy: MergeStrategy::Namespaced,
    },

    // New: bootstrap/tooling configs
    MergeSpec {
        relative_path: "config/tools.json",
        raw_key: "tools",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/bootstrap-tools.json",
        raw_key: "bootstrap_tools",
        strategy: MergeStrategy::Namespaced,
    },
    MergeSpec {
        relative_path: "config/bootstrap-state.json",
        raw_key: "bootstrap_state",
        strategy: MergeStrategy::Namespaced,
    },

    // New: provider defaults
    MergeSpec {
        relative_path: "config/providers/default.yaml",
        raw_key: "provider_defaults",
        strategy: MergeStrategy::Namespaced,
    },
];
