use noa_core::self_improve::{RollbackManager, SnapshotManager};
use serde_json::json;
use std::time::Instant;
use tempfile::tempdir;

#[path = "../benchmarks/common.rs"]
mod common;
use common::{record_result, BenchmarkResult, BenchmarkStatus};

#[test]
fn sc010_rollback_path_validation() {
    let tmp = tempdir().expect("create temp snapshot dir");
    let snapshot_mgr = SnapshotManager::new(tmp.path());
    let snapshot = snapshot_mgr
        .create("baseline", json!({ "version": 1, "state": "ok" }))
        .expect("snapshot creation");
    let rollback = RollbackManager::new(snapshot_mgr);

    let start = Instant::now();
    let outcome = rollback.rollback(&snapshot.id);
    let duration = start.elapsed();

    let mut result = BenchmarkResult::new(
        "SC-010",
        "rollback path validation (100% coverage)",
        duration,
        1_000,
    );

    match outcome {
        Ok(ok) if ok.restored => {
            // Negative path: missing snapshot should error
            let missing = rollback.rollback("missing-id").is_err();
            if !missing {
                result.status = BenchmarkStatus::Failed;
                result.notes = Some("missing snapshot did not error".to_string());
            } else {
                result.notes = Some(format!("restored_id={} message={}", ok.snapshot_id, ok.message));
            }
        }
        Ok(_) | Err(_) => {
            result.status = BenchmarkStatus::Failed;
            result.notes = Some("rollback failed to restore snapshot".to_string());
        }
    }

    record_result(&result);
    assert!(
        result.passed(),
        "SC-010 failed: {:?} ({}ms)",
        result.notes,
        result.duration_ms
    );
}
