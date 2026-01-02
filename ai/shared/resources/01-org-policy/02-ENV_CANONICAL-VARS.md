# Canonical Environment Variables Registry

| Status     | Version | FR              |
|------------|---------|-----------------|
| **Active** | 1.0.0   | FR-001, FR-002  |

## Purpose

This document is the **single source of truth** for all NOA environment variables. Platform-specific env scripts (`noa-env.ps1`, `.noa-env`, `noa-env.cmd`, `noa-env.fish`) are **auto-generated** from this registry via `scripts/gen-env-scripts.js`.

> **Never edit env scripts directly.** Edit this file, then run `node scripts/gen-env-scripts.js` to regenerate.

## Related Policies

- [02-ENV_HOME-vs-ROOT.md](02-ENV_HOME-vs-ROOT.md) - NOA_ROOT vs NOA_HOME distinction
- [02-ENV_CONTAINMENT.md](02-ENV_CONTAINMENT.md) - FR-001 AppData containment
- [03-CONFIG_PACKAGE-MANAGER.md](03-CONFIG_PACKAGE-MANAGER.md) - pnpm containment

---

## Variable Registry

All variables use `${NOA_ROOT}` as the base anchor. For hardware detection, see Hardware Detection section.

```yaml
# ============================================================================
# CORE PATHS - Foundation anchors
# ============================================================================
variables:
  # --- Anchors ---
  - name: NOA_ROOT
    description: Ecosystem base anchor point - auto-detected from script location
    default: "${SCRIPT_DIR}"
    required: true
    scope: global
    autodetect: true
    
  - name: NOA_HOME
    description: Active instance directory (may equal NOA_ROOT for single-folder installs)
    default: "${NOA_ROOT}"
    required: true
    scope: instance
    
  - name: NOA_ENV
    description: Environment mode (development, staging, production)
    default: "development"
    required: false
    scope: runtime
    
  - name: NOA_VERSION
    description: Current NOA version string
    default: "1.0.0"
    required: false
    scope: runtime
    
  - name: NOA_INSTANCE_ID
    description: Unique instance identifier for multi-instance deployments
    default: ""
    required: false
    scope: instance

  # ============================================================================
  # HARDWARE DETECTION - Platform/device identification
  # ============================================================================
  - name: NOA_DEVICE_CLASS
    description: Device type classification
    default: "desktop"
    required: false
    scope: runtime
    autodetect: true
    enum: ["desktop", "server", "mobile", "embedded", "xr"]
    
  - name: NOA_COMPUTE_PROFILE
    description: Compute acceleration profile
    default: "cpu-only"
    required: false
    scope: runtime
    autodetect: true
    enum: ["cpu-only", "cuda", "metal", "rocm", "vulkan", "directml"]
    
  - name: NOA_PLATFORM
    description: Operating system platform
    default: ""
    required: false
    scope: runtime
    autodetect: true
    enum: ["windows", "linux", "darwin", "android", "ios"]
    
  - name: NOA_ARCH
    description: CPU architecture
    default: ""
    required: false
    scope: runtime
    autodetect: true
    enum: ["x64", "arm64", "x86", "arm"]

  # ============================================================================
  # DIRECTORY STRUCTURE - XDG-compliant paths
  # ============================================================================
  - name: NOA_BIN
    description: Executable binaries
    default: "${NOA_ROOT}/bin"
    required: true
    scope: global
    path_add: true
    
  - name: NOA_CONFIG
    description: Configuration files
    default: "${NOA_ROOT}/config"
    required: true
    scope: global
    
  - name: NOA_DATA
    description: Persistent data storage
    default: "${NOA_ROOT}/data"
    required: true
    scope: global
    
  - name: NOA_CACHE
    description: Cache directory (regenerable)
    default: "${NOA_ROOT}/cache"
    required: true
    scope: global
    
  - name: NOA_LOGS
    description: Log files
    default: "${NOA_ROOT}/logs"
    required: true
    scope: global
    
  - name: NOA_TMP
    description: Temporary files
    default: "${NOA_ROOT}/tmp"
    required: true
    scope: global
    
  - name: NOA_OPT
    description: Optional/portable tools
    default: "${NOA_ROOT}/opt"
    required: true
    scope: global
    
  - name: NOA_ETC
    description: System configuration (XDG config)
    default: "${NOA_ROOT}/etc"
    required: true
    scope: global
    
  - name: NOA_LIB
    description: Shared libraries
    default: "${NOA_ROOT}/lib"
    required: true
    scope: global
    
  - name: NOA_SCRIPTS
    description: Utility scripts
    default: "${NOA_ROOT}/scripts"
    required: true
    scope: global
    path_add: true

  # ============================================================================
  # AI SUBSYSTEM
  # ============================================================================
  - name: NOA_AI
    description: AI subsystem root
    default: "${NOA_ROOT}/ai"
    required: true
    scope: global
    
  - name: NOA_AI_SHARED
    description: Shared AI resources (canonical provider location)
    default: "${NOA_AI}/shared"
    required: true
    scope: global
    
  - name: NOA_AI_PROVIDERS
    description: AI provider configurations (CANONICAL - symlinked from config/providers)
    default: "${NOA_AI}/providers"
    required: true
    scope: global

  # ============================================================================
  # APPDATA REDIRECTION (FR-001 Containment)
  # ============================================================================
  - name: APPDATA
    description: Windows roaming AppData (redirected)
    default: "${NOA_ROOT}/data/appdata/roaming"
    required: true
    scope: global
    platform: windows
    override: true
    
  - name: LOCALAPPDATA
    description: Windows local AppData (redirected)
    default: "${NOA_ROOT}/data/appdata/local"
    required: true
    scope: global
    platform: windows
    override: true
    
  - name: TEMP
    description: Temporary directory (redirected)
    default: "${NOA_ROOT}/tmp"
    required: true
    scope: global
    override: true
    
  - name: TMP
    description: Temporary directory (alias)
    default: "${NOA_ROOT}/tmp"
    required: true
    scope: global
    override: true

  # ============================================================================
  # XDG BASE DIRECTORY (Unix compatibility)
  # ============================================================================
  - name: XDG_DATA_HOME
    description: User data directory
    default: "${NOA_ROOT}/data"
    required: true
    scope: global
    override: true
    
  - name: XDG_CONFIG_HOME
    description: User config directory
    default: "${NOA_ROOT}/etc"
    required: true
    scope: global
    override: true
    
  - name: XDG_CACHE_HOME
    description: User cache directory
    default: "${NOA_ROOT}/cache"
    required: true
    scope: global
    override: true
    
  - name: XDG_STATE_HOME
    description: User state directory
    default: "${NOA_ROOT}/data/state"
    required: true
    scope: global
    override: true

  # ============================================================================
  # PACKAGE MANAGER (pnpm canonical)
  # ============================================================================
  - name: PNPM_HOME
    description: pnpm installation directory
    default: "${NOA_ROOT}/opt/pnpm"
    required: true
    scope: global
    path_add: true
    
  - name: PNPM_STORE_DIR
    description: pnpm content-addressable store
    default: "${NOA_ROOT}/cache/pnpm"
    required: true
    scope: global

  # ============================================================================
  # NODE.JS
  # ============================================================================
  - name: NOA_NODE
    description: Node.js installation
    default: "${NOA_ROOT}/opt/node"
    required: true
    scope: global
    path_add: true
    
  - name: NODE_PATH
    description: Node.js module path
    default: "${NOA_NODE}/node_modules"
    required: true
    scope: global
    
  - name: npm_config_prefix
    description: npm global prefix (contained)
    default: "${NOA_NODE}"
    required: true
    scope: global
    
  - name: npm_config_cache
    description: npm cache directory (contained)
    default: "${NOA_ROOT}/cache/npm"
    required: true
    scope: global

  # ============================================================================
  # RUST
  # ============================================================================
  - name: RUSTUP_HOME
    description: Rustup installation
    default: "${NOA_ROOT}/opt/rust/rustup"
    required: true
    scope: global
    
  - name: CARGO_HOME
    description: Cargo installation
    default: "${NOA_ROOT}/opt/rust/cargo"
    required: true
    scope: global
    path_add_subdir: "bin"

  # ============================================================================
  # GO
  # ============================================================================
  - name: GOROOT
    description: Go installation root
    default: "${NOA_ROOT}/opt/go"
    required: true
    scope: global
    path_add_subdir: "bin"
    
  - name: GOPATH
    description: Go workspace
    default: "${NOA_ROOT}/opt/go/workspace"
    required: true
    scope: global
    
  - name: GOBIN
    description: Go binary output
    default: "${GOPATH}/bin"
    required: true
    scope: global
    path_add: true
    
  - name: GOCACHE
    description: Go build cache
    default: "${NOA_ROOT}/cache/go"
    required: true
    scope: global
    
  - name: GOMODCACHE
    description: Go module cache
    default: "${NOA_ROOT}/cache/go/mod"
    required: true
    scope: global

  # ============================================================================
  # PYTHON
  # ============================================================================
  - name: PYTHONHOME
    description: Python installation
    default: "${NOA_ROOT}/opt/python"
    required: true
    scope: global
    path_add: true
    
  - name: VIRTUAL_ENV
    description: Python virtual environment
    default: "${NOA_ROOT}/opt/venv"
    required: true
    scope: global
    path_add_subdir: "Scripts"
    path_add_subdir_unix: "bin"
    
  - name: PIP_CACHE_DIR
    description: pip cache directory
    default: "${NOA_ROOT}/cache/pip"
    required: true
    scope: global

  # ============================================================================
  # OBSERVABILITY (OpenTelemetry)
  # ============================================================================
  - name: OTEL_EXPORTER_OTLP_ENDPOINT
    description: OpenTelemetry collector endpoint
    default: "http://localhost:4317"
    required: false
    scope: runtime
    
  - name: OTEL_SERVICE_NAME
    description: Service name for telemetry
    default: "noa"
    required: false
    scope: runtime
    
  - name: OTEL_TRACES_SAMPLER
    description: Trace sampling strategy
    default: "parentbased_traceidratio"
    required: false
    scope: runtime

  # ============================================================================
  # SERVICES (MinIO, Qdrant, etc.)
  # ============================================================================
  - name: NOA_MINIO_ROOT_USER
    description: MinIO admin username
    default: "minioadmin"
    required: false
    scope: service
    secret: true
    
  - name: NOA_MINIO_ROOT_PASSWORD
    description: MinIO admin password
    default: ""
    required: false
    scope: service
    secret: true
    
  - name: NOA_QDRANT_URL
    description: Qdrant vector store URL
    default: "http://localhost:6333"
    required: false
    scope: service

  # ============================================================================
  # CONDA / ML ENVIRONMENTS
  # ============================================================================
  - name: NOA_CONDA
    description: Conda/micromamba installation
    default: "${NOA_ROOT}/opt/conda"
    required: false
    scope: global
    
  - name: NOA_CONDA_ENV
    description: Default conda environment
    default: "${NOA_CONDA}/envs/noa"
    required: false
    scope: global
```

---

## Hardware Detection Logic

The bootstrap script (`scripts/bootstrap.js`) sets these automatically:

### NOA_PLATFORM

```javascript
const platform = process.platform; // 'win32' | 'darwin' | 'linux' | 'android'
// Mapped to: 'windows' | 'darwin' | 'linux' | 'android' | 'ios'
```

### NOA_ARCH

```javascript
const arch = process.arch; // 'x64' | 'arm64' | 'ia32' | 'arm'
```

### NOA_DEVICE_CLASS

Detection order:

1. Check for XR headset APIs → `xr`
2. Check `/proc/device-tree/model` for Raspberry Pi → `embedded`
3. Check Android/iOS runtime → `mobile`
4. Check if running headless or in container → `server`
5. Default → `desktop`

### NOA_COMPUTE_PROFILE

Detection order:

1. Check `nvidia-smi` available → `cuda`
2. Check Metal framework (macOS) → `metal`
3. Check ROCm (`rocm-smi`) → `rocm`
4. Check DirectML (Windows) → `directml`
5. Check Vulkan compute → `vulkan`
6. Default → `cpu-only`

---

## Generation

To regenerate all env scripts from this source:

```bash
node scripts/gen-env-scripts.js
```

This produces:

- `noa-env.ps1` (PowerShell)
- `.noa-env` (Bash/Zsh)
- `noa-env.cmd` (Windows CMD)
- `noa-env.fish` (Fish shell)
- `noa-env.zsh` (Zsh with completions)

---

## Changelog

| Version | Date       | Changes                                                  |
|---------|------------|----------------------------------------------------------|
| 1.0.0   | 2025-12-19 | Initial registry; hardware detection; pnpm containment   |
