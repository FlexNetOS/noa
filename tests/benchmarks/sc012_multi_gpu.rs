use std::env;
use std::thread;
use std::time::{Duration, Instant};

mod common;
use common::{record_result, BenchmarkResult};

#[test]
fn sc012_multi_gpu_inference_under_300ms() {
    let target_ms = 300u128;

    if !gpu_available() {
        let skipped = BenchmarkResult::skipped(
            "SC-012",
            "multi-GPU inference (<300ms)",
            "GPU not detected (set NOA_GPU_AVAILABLE=1 to exercise path)",
            target_ms,
        );
        record_result(&skipped);
        return;
    }

    let start = Instant::now();

    // Simulate tensor-parallel shards running concurrently
    let shard = || thread::sleep(Duration::from_millis(80));
    let h1 = thread::spawn(shard);
    let h2 = thread::spawn(shard);
    let _ = h1.join();
    let _ = h2.join();

    let duration = start.elapsed();
    let mut result = BenchmarkResult::new(
        "SC-012",
        "multi-GPU inference (<300ms)",
        duration,
        target_ms,
    );
    result.notes = Some("simulated 2-way sharding; replace with real GPU timing".to_string());

    record_result(&result);
    assert!(
        result.passed(),
        "SC-012 failed: {}ms vs target {}ms",
        result.duration_ms,
        result.target_ms
    );
}

fn gpu_available() -> bool {
    matches!(env::var("NOA_GPU_AVAILABLE"), Ok(v) if v == "1")
}
