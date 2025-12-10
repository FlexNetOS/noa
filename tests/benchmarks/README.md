# Performance Benchmarks (T411)

Scope: timing harnesses for critical paths (init, inference, memory recall) and throughput sanity checks.

Status: Placeholder. Implement benches using `criterion` or simple wall-clock timers once services are runnable.

Suggested targets:
- Init timing (SC-001) – bootstrap + `noa start` cold/warm timings.
- Inference latency (SC-002/SC-011/SC-012) – mock model adapters until real runtime is available.
- Memory recall (SC-003) – query latency under sample load.

Run (future):
```bash
cd sys/core
cargo bench
```
