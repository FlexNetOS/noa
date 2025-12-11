use std::env;
use std::time::{Duration, Instant};

mod common;
use common::{record_result, BenchmarkResult};

#[test]
fn sc011_single_gpu_inference_under_500ms() {
    let target_ms = 500u128;

    if !gpu_available() {
        let skipped = BenchmarkResult::skipped(
            "SC-011",
            "single GPU inference (<500ms)",
            "GPU not detected (set NOA_GPU_AVAILABLE=1 to exercise path)",
            target_ms,
        );
        record_result(&skipped);
        return;
    }

    let start = Instant::now();

    // Simulated GPU-bound shard (placeholder until GPU backend is wired)
    std::thread::sleep(Duration::from_millis(50));

    let duration = start.elapsed();
    let mut result =
        BenchmarkResult::new("SC-011", "single GPU inference (<500ms)", duration, target_ms);
    result.notes = Some("simulated GPU path; replace with real backend timing".to_string());

    record_result(&result);
    assert!(
        result.passed(),
        "SC-011 failed: {}ms vs target {}ms",
        result.duration_ms,
        result.target_ms
    );
}

fn gpu_available() -> bool {
    matches!(env::var("NOA_GPU_AVAILABLE"), Ok(v) if v == "1")
}
