use noa_core::services::InitService;
use std::time::Instant;
use tempfile::tempdir;

mod common;
use common::{record_result, BenchmarkResult, BenchmarkStatus};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sc001_system_initialization_under_60s() {
    let tmp = tempdir().expect("failed to create temp noa root");
    let start = Instant::now();

    let init_outcome = InitService::initialize(tmp.path(), true).await;
    let duration = start.elapsed();

    let mut result =
        BenchmarkResult::new("SC-001", "system initialization (<60s)", duration, 60_000);

    match init_outcome {
        Ok(details) => {
            if !details.errors.is_empty() {
                result.status = BenchmarkStatus::Failed;
                result.notes = Some(format!(
                    "init completed with {} errors",
                    details.errors.len()
                ));
            } else {
                result.notes = Some(format!(
                    "dirs={} configs={} db={}",
                    details.directories_created,
                    details.configs_generated,
                    details.database_initialized
                ));
            }
        }
        Err(err) => {
            result.status = BenchmarkStatus::Failed;
            result.notes = Some(format!("init failed: {err}"));
        }
    }

    record_result(&result);
    assert!(
        result.passed(),
        "SC-001 failed: {}ms vs target {}ms ({:?})",
        result.duration_ms,
        result.target_ms,
        result.notes
    );
}
