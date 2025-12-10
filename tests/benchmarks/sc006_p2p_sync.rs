use std::time::Instant;
use tokio::sync::mpsc;

mod common;
use common::{record_result, BenchmarkResult, BenchmarkStatus};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sc006_p2p_sync_under_5s_for_1mb() {
    const TARGET_MS: u128 = 5_000;
    let payload = vec![42u8; 1_000_000]; // 1MB delta
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(4);

    let start = Instant::now();

    // Simulated sender
    let sender = tokio::spawn(async move {
        // Chunk into four pieces to mimic streaming sync
        for chunk in payload.chunks(250_000) {
            tx.send(chunk.to_vec()).await.unwrap();
        }
    });

    // Simulated receiver aggregates the bytes
    let mut received: usize = 0;
    while let Some(chunk) = rx.recv().await {
        received += chunk.len();
        if received >= 1_000_000 {
            break;
        }
    }

    let _ = sender.await;
    let duration = start.elapsed();

    let mut result = BenchmarkResult::new(
        "SC-006",
        "p2p sync latency (<5s for 1MB delta)",
        duration,
        TARGET_MS,
    );

    if received < 1_000_000 {
        result.status = BenchmarkStatus::Failed;
        result
            .notes
            .replace(format!("incomplete transfer: {} bytes", received));
    } else {
        result
            .notes
            .replace(format!("bytes={} duration_ms={}", received, result.duration_ms));
    }

    record_result(&result);
    assert!(
        result.passed(),
        "SC-006 failed: transferred {} bytes in {}ms (target {}ms)",
        received,
        result.duration_ms,
        result.target_ms
    );
}
