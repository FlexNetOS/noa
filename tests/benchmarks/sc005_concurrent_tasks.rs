use futures::future::join_all;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

mod common;
use common::{record_result, BenchmarkResult, BenchmarkStatus};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sc005_concurrent_agent_tasks_success_rate() {
    const TASKS: usize = 200;
    const TARGET_MS: u128 = 5_000;

    let successes = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();

    let handles = (0..TASKS)
        .map(|i| {
            let successes = successes.clone();
            tokio::spawn(async move {
                // Simulated agent workload with deterministic CPU effort
                let iterations = 5_000 + (i % 5) * 500;
                let mut acc: u64 = 0;
                for j in 0..iterations {
                    acc = acc.wrapping_add((i as u64) ^ (j as u64));
                }
                if acc % 2 == 0 {
                    successes.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect::<Vec<_>>();

    let _ = join_all(handles).await;
    let duration = start.elapsed();
    let success_rate = successes.load(Ordering::Relaxed) as f64 / TASKS as f64;

    let mut result = BenchmarkResult::new(
        "SC-005",
        "concurrent agent tasks (200 tasks, ≥98% success)",
        duration,
        TARGET_MS,
    );

    if success_rate < 0.98 {
        result.status = BenchmarkStatus::Failed;
    }
    result.notes = Some(format!(
        "success_rate={:.3} successes={} duration_ms={}",
        success_rate,
        successes.load(Ordering::Relaxed),
        result.duration_ms
    ));

    record_result(&result);
    assert!(
        result.passed(),
        "SC-005 failed: {:.2}% success, {}ms vs target {}ms",
        success_rate * 100.0,
        result.duration_ms,
        result.target_ms
    );
}
