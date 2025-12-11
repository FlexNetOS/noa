use noa_core::governance::{
    GovernanceDecision, GovernanceHarness, DecisionVerdict, SnapshotService, RollbackValidator,
};
use serde_json::json;
use std::path::PathBuf;
use tempfile::tempdir;
use tokio::fs;

fn make_decision(agent: &str, action: &str, rationale: &str) -> GovernanceDecision {
    GovernanceDecision::new(
        agent,
        action,
        rationale,
        json!({ "action": action }),
        vec!["3.10".to_string()],
    )
}

#[tokio::test]
async fn governance_allows_safe_action() {
    let dir = tempdir().unwrap();
    let audit_path = dir.path().join("audit.log");
    let harness = GovernanceHarness::new(audit_path);
    harness.init_defaults().await;

    let decision = make_decision(
        "agent-safe",
        "update-docs",
        "Act justly and protect the vulnerable; do no harm and be truthful.",
    );
    let (outcome, score) = harness.evaluate(decision).await.unwrap();

    assert_eq!(outcome.final_verdict, DecisionVerdict::Allow);
    assert!(score >= -0.5);
}

#[tokio::test]
async fn governance_denies_risky_action() {
    let dir = tempdir().unwrap();
    let audit_path = dir.path().join("audit.log");
    let harness = GovernanceHarness::new(audit_path);
    harness.init_defaults().await;

    let decision = make_decision(
        "agent-risk",
        "alter-logs",
        "Plan to exploit users and tamper with safety logs.",
    );
    let (outcome, score) = harness.evaluate(decision).await.unwrap();

    assert_eq!(outcome.final_verdict, DecisionVerdict::Deny);
    assert!(score <= -0.5);
}

#[tokio::test]
async fn snapshot_validation_happy_path() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.yaml");
    fs::write(&file_path, b"key: value").await.unwrap();

    let snapshot_root = dir.path().join("snapshots");
    let service = SnapshotService::new(snapshot_root);
    let record = service
        .create_snapshot(
            "config change",
            "tester",
            &json!({"change": "config"}),
            &[file_path.clone()],
        )
        .unwrap();

    let validator = RollbackValidator::new(SnapshotService::new(dir.path().join("snapshots")));
    let assessment = validator.validate(&record.id).unwrap();
    assert!(assessment.valid);
    assert!(assessment.missing_artifacts.is_empty());
}

#[tokio::test]
async fn snapshot_validation_detects_missing_artifacts() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.yaml");
    fs::write(&file_path, b"key: value").await.unwrap();

    let snapshot_root = dir.path().join("snapshots");
    let service = SnapshotService::new(snapshot_root.clone());
    let record = service
        .create_snapshot(
            "config change",
            "tester",
            &json!({"change": "config"}),
            &[file_path.clone()],
        )
        .unwrap();

    // Remove the file to force a missing artifact
    fs::remove_file(&file_path).await.unwrap();

    let validator = RollbackValidator::new(SnapshotService::new(snapshot_root));
    let assessment = validator.validate(&record.id).unwrap();
    assert!(!assessment.valid);
    assert!(!assessment.missing_artifacts.is_empty());
}
