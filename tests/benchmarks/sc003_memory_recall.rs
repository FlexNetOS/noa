use noa_core::db::repositories::memory_repository::MemoryType;
use noa_core::services::MemoryService;
use std::collections::HashSet;
use std::time::Instant;

mod common;
use common::{record_result, BenchmarkResult};

const MEMORY_SCHEMA: &str = r#"
CREATE TABLE memory (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    type TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata TEXT,
    source_agent TEXT,
    parent_id TEXT,
    tags TEXT,
    embedding_id TEXT,
    checksum TEXT NOT NULL
);"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sc003_memory_recall_under_500ms() {
    let mut conn = noa_core::db::Connection::open_in_memory().expect("open in-memory db");
    conn.execute_batch(MEMORY_SCHEMA)
        .expect("create memory schema");

    let service = MemoryService::new(conn);
    let mut ids = Vec::new();

    for idx in 0..200 {
        let content = format!("memory item {}", idx);
        let id = service
            .create(
                MemoryType::Interaction,
                content,
                None,
                None,
                None,
                HashSet::new(),
            )
            .await
            .expect("insert memory");
        ids.push(id);
    }

    let start = Instant::now();
    for i in 0..50 {
        let id = &ids[i];
        let memory = service.get(id).expect("query memory");
        assert!(memory.is_some(), "memory should exist");
    }
    let duration = start.elapsed();

    let mut result =
        BenchmarkResult::new("SC-003", "memory recall latency (<500ms)", duration, 500);
    result.notes = Some(format!(
        "retrieved=50 total_entries={} duration_ms={}",
        ids.len(),
        result.duration_ms
    ));

    record_result(&result);
    assert!(
        result.passed(),
        "SC-003 failed: {}ms vs target {}ms",
        result.duration_ms,
        result.target_ms
    );
}
