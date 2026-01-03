# Error Specialist Subagents

Each subagent owns a class of failures with detectors, auto-remediations, and handoff rules.

## RetryableIOError Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## NonRetryableIOError Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## NetworkTimeout Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## DNSFailure Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## CircuitBreakerOpen Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## RateLimited Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## AuthTokenExpired Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## Unauthorized Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## ForbiddenEgress Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## OPA_Deny Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SecretMissing Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SecretDecryptionFailed Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SpaceQuotaExceeded Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## InodeExhaustion Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## FileLockContention Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## BuildCacheMiss Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## BuildKitUnavailable Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## ContainerStartFailure Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## OOM Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## GPU_DriverMismatch Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## CUDA_NotFound Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## CPU_InstructionUnsupported Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## WSL_PathMapping Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## Windows_ACL Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SymlinkPermission Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## TLSHandshakeFailure Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## ClockSkew Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SchemaMismatch Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SBOMGenerationError Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## VulnDBUnavailable Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## LicensePolicyViolation Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## ExportControlFlag Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## SemgrepRuleFailure Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## GitleaksFinding Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## ProvenanceHashMismatch Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

## LineageLoopDetected Subagent
- **Detectors:** logs, metrics, traces, exit codes
- **Auto-remediations:** retry/backoff, alternate path, degrade/service mirror, advise board
- **Escalation:** Trifecta Court if policy gates or repeated failures
- **Telemetry:** counters, latencies, success rates, MTTR

