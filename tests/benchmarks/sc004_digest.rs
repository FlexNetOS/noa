use noa_core::db::{init_database, Connection};
use noa_core::db::repositories::digest_repository::DigestSourceType;
use noa_core::services::DigestService;
use std::time::Instant;
use tempfile::tempdir;

mod common;
use common::{record_result, BenchmarkResult};

const DIGEST_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS digest_source (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,
    uri TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    last_digest TEXT,
    version TEXT,
    profile TEXT,
    sbom TEXT,
    security_report TEXT,
    stats TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_digest_source_uri ON digest_source(uri);
"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sc004_digest_pipeline_under_30_minutes() {
    let tmp = tempdir().expect("create temp dir for digest db");
    let db_path = tmp.path().join("digest.db");

    bootstrap_digest_schema(&db_path);

    let service = DigestService::new(&db_path);
    let start = Instant::now();

    let digest_id = service
        .digest_source("https://example.com/repo.git", DigestSourceType::Repository)
        .await
        .expect("digest pipeline execution");

    let duration = start.elapsed();
    let mut result = BenchmarkResult::new(
        "SC-004",
        "digest pipeline throughput (<30min for 10K files)",
        duration,
        30 * 60 * 1000, // 30 minutes target in milliseconds
    );
    result.notes = Some(format!("digest_id={digest_id} duration_ms={}", result.duration_ms));

    record_result(&result);
    assert!(
        result.passed(),
        "SC-004 failed: {}ms vs target {}ms",
        result.duration_ms,
        result.target_ms
    );
}

fn bootstrap_digest_schema(db_path: &std::path::Path) {
    let conn: Connection = init_database(db_path).expect("open digest database");
    conn.execute_batch(DIGEST_SCHEMA)
        .expect("bootstrap digest schema");
}
