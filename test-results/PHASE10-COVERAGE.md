# Phase 10 Coverage Snapshot (US8 Self-Improvement)

## Scope
- Source: specs/001-noa-seed-foundation/tasks.md, Phase 10 (T307-T361, T765-T770).
- Modules: graphs, CECCA, knowledge capsules, self-improve, policy, autonomy loop, predictive, VHDX, CLI improve.

## Artifacts Mapped
- Graphs (T307-T316): `sys/core/src/graphs/*.rs`; tests: `graphs::base::tests::*`.
- CECCA Cells (T317-T325): `sys/core/src/cecca/cells/*.rs`; new tests added for constitution (`constitution.rs`).
- Knowledge Capsules (T326-T331): `sys/core/src/knowledge/*.rs`; new tests added for capsule index (`kidx.rs`).
- Self-Analysis & Safety (T338-T343): `sys/core/src/self_improve/{metrics,analyzer,proposals,snapshot,test_runner,rollback}.rs`.
- Evidence Policy (T344-T347): `sys/core/src/policy/{truth_sources,hard_stop,triple_verify,gap_hunt}.rs`.
- Autonomous Loop (T348-T352): `sys/core/src/autonomy/{ampk,autonomy_loop,scheduler}.rs` + healing loop `src/healing/*`.
- Human Co-Improvement (T353-T354): `sys/core/src/self_improve/{approval,audit}.rs`.
- VHDX (T355-T357): `sys/core/src/vhdx/{packaging,nested,snapshot}.rs`.
- CLI Improve (T358-T361): `sys/core/src/cli/improve.rs`.
- Predictive (T765-T770): `sys/core/src/predict/{patterns,failure_analysis,value_evaluator,resource_predictor,priority_predictor,risk_assessment}.rs`.
- GPU/Phase10 verification: `src/neural/phase10_verification_test.rs`.

## Test Execution
- Command: `cd sys/core; cargo test -p noa-core --lib`
- Result: PASS (217 passed, 3 ignored); warnings present.

## Gaps / Follow-ups
- Many Phase 10 modules lack dedicated assertions (CECCA cells beyond constitution, knowledge schema/metrics, policy rules, predictive engines, VHDX, CLI improve).
- Warning cleanup still needed across modules (unused imports/fields).
- No hashes emitted; no E2E/self-improve flow tests.

## Next Steps
1) Add targeted unit tests for CECCA quorum/truth_gate, knowledge schema/metrics, policy rules, predictive engines, and VHDX snapshot/packaging behaviors.
2) Reduce warning noise (unused imports/vars) focusing on Phase 10 modules and GPU phase10 tests.
3) Generate hashes once coverage tightened (`sha256sum` into HASHES.txt) and update this snapshot.
