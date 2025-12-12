use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::Instant;

#[path = "../benchmarks/common.rs"]
mod common;
use common::{record_result, BenchmarkResult};

#[test]
fn sc009_cross_platform_consistency() {
    let start = Instant::now();

    // Deterministic hash calculation should not vary by platform
    let sample = b"cross-platform-consistency";
    let digest = Sha256::digest(sample);
    let hex = format!("{:x}", digest);
    let expected = "3e34493d4e8b709bb7fb17dd59bcfcf480c5c2b89404bef8ffb5fcdd4e22e1a3";

    // Path normalization check to ensure consistent separators
    let normalized = PathBuf::from_iter(["sandbox", "plane", "config"])
        .to_string_lossy()
        .replace('\\', "/");

    let duration = start.elapsed();
    let mut result = BenchmarkResult::new(
        "SC-009",
        "cross-platform consistency (hash + paths)",
        duration,
        1_000,
    );
    if hex != expected || normalized != "sandbox/plane/config" {
        result.notes = Some(format!(
            "hash_ok={} path_ok={}",
            hex == expected,
            normalized
        ));
        result.status = common::BenchmarkStatus::Failed;
    } else {
        result.notes = Some(format!("hash={} normalized_path={}", hex, normalized));
    }

    record_result(&result);
    assert!(result.passed(), "SC-009 failed: {:?}", result.notes);
}
