# Kernel Mode Performance Trade-offs (CHK007, CHK032)

- Native: baseline throughput; zero virtualization overhead; rely on NKAL for capability gating only.
- Sandbox: expected +5-10% CPU overhead and slower I/O due to sandbox policy; recommended for untrusted plugins during development.
- Container: expected +10-15% CPU overhead and +15-20% disk overhead from layered filesystems; GPU passthrough depends on host runtime (documented in mount config).
- VM: expected +20-35% CPU overhead and higher memory footprint; isolation highest, use when boundary policy requires full separation.
- Overhead acceptance bands (CHK032): alert if sandbox >15%, container >25%, VM >40% relative to native for equivalent workload; exceeding bands triggers rollback to previous mode.
- Trade-off rule: pick the highest isolation that stays within the acceptable overhead band while still satisfying capability policy and mount availability.
