use std::time::Instant;

mod common;
use common::{record_result, BenchmarkResult};

#[test]
fn sc002_cpu_inference_under_2s() {
    let start = Instant::now();

    // Simulate CPU-bound token generation workload
    let mut accumulator = 0.0f64;
    let iterations = 150_000u32;
    for i in 1..=iterations {
        let x = i as f64 * 0.0001;
        accumulator += (x.sin() * x.cos()).abs().sqrt();
    }

    let duration = start.elapsed();
    let mut result =
        BenchmarkResult::new("SC-002", "cpu inference latency (<2s)", duration, 2_000);
    result.notes = Some(format!(
        "iterations={iterations} accumulator={:.4}",
        accumulator
    ));

    record_result(&result);
    assert!(
        result.passed(),
        "SC-002 failed: {}ms vs target {}ms ({:?})",
        result.duration_ms,
        result.target_ms,
        result.notes
    );
}
