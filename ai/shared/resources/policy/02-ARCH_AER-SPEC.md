# noa – Agentic Environment Runtime (AER) v2  
### High-Level Architecture Specification

> **Implementation**: See [../../specs/001-noa-seed-foundation/](../../specs/001-noa-seed-foundation/) for the detailed implementation specification, tasks, and data model derived from this architecture.
>
> **Related Docs**: [Policy Index](./README.md) | [Constitution](./01_CONSTITUTION.md) | [AGENT.md](../../specs/AGENT.md)

---

## 1. Overview

**noa** is a cross-platform, agentic runtime that behaves like:

- A **personal OS layer** for a single `$user`, spanning all of their devices.
- A **monolith with internal microservices** (single deployable app, logically composed of many services).
- A **host for other apps** (desktop/mobile) that:
  - Can run inside noa-controlled environments.
  - May share environments or have fully isolated ones.
- A **user hive mind**:
  - One logical “brain” per `$user` across all devices.
  - Optional participation in a **global p2p / central model** with explicit consent.

Key properties:

- Runs on: Linux, Windows 11, macOS, servers, Android, iOS, tablets, glasses, XR devices.
- Uses the **host kernel directly** (no visible hypervisor GUI).
- Provides **VM-like isolation** + **container-like UX**:
  - Independent environments.
  - Direct host kernel access via adapters.
  - Apps look & feel native.
- Uses **2–3 small (<3B) models** via **llama.cpp**, plus optional additional small models, to:
  - Orchestrate the environment.
  - Generate & refactor tools.
  - Enforce safety and policy.
- Treats **binaries, packages, libraries, and micro-kernels** as dynamic, versioned artifacts.
- Uses **llama.cpp agents** that can be **spliced, merged, and updated** for dynamic optimization.

---

## 2. Design Goals & Constraints

### 2.1 Goals

1. **Cross-platform**  
   noa installs like a normal app on all major OSes, no special hypervisor UX.

2. **Host-kernel based**  
   All execution uses the **host kernel** (processes, threads, syscalls), mediated through noa’s adapters.

3. **User Hive Mind**  
   - One **logical brain** per `$user`, spanning all their devices.
   - Shared memory, policies, and models (with device-specific constraints).

4. **Resource Mesh**  
   - All `$user` devices form a **private resource cluster**:
     - Compute, storage, memory pooled at the logical level.
   - Optional **donation of spare resources** to a global network (with policy & consent).

5. **App Hosting**  
   - noa can **host other apps**:
     - Some apps share common envs.
     - Some get fully isolated envs.
   - Retain all app features while routing their data/traffic through noa where possible.

6. **Sensor-driven learning**  
   - Mic & camera can be **always-on (configurable)** for the `$user` to train their own hive mind.
   - Derived features, not raw data, are used for most learning.
   - Optional, consent-based contribution to global models.

7. **Monolith with Microservices**  
   - Single deployable binary or app bundle named **noa**.
   - Internally structured as microservices communicating over a message bus.
   - Dynamic modules, hot-swappable where supported.

### 2.2 Non-Goals (Hard Reality Constraints)

- Not intended to **bypass OS security** or root/jailbreak devices.
- On stock iOS/Android, noa **cannot literally host all other apps** at OS level:
  - It can act as data/router/proxy where APIs allow.
  - Full “app hypervisor” behavior is realistic on Linux, controlled Windows/macOS environments, and servers.

---

## 3. System Context & Concepts

### 3.1 Core Terms

- **$user** – A human identity, with one logical hive mind across devices.
- **Device** – Laptop, desktop, phone, tablet, glasses, server, etc.
- **noa Instance** – The running noa app on a specific device.
- **User Hive** – The logical, per-user brain: memories, preferences, policies, local personalization.
- **Global Network** – Central / semi-central / p2p infrastructure for cross-user model training & coordination.
- **Module** – Versioned, immutable artifact: binary, package, library, tool, microkernel, agent.
- **Environment (env)** – A logical runtime context: capabilities, policy, process set, data view.

---

## 4. High-Level Architecture

```text
+----------------------------------------------------------------+
|                           noa (per device)                     |
|                                                                |
|  +--------------------- Monolith Binary --------------------+  |
|  |                                                         |  |
|  |  [ Host Adapter Layer ]                                 |  |
|  |          |                                              |  |
|  |          v                                              |  |
|  |  [ noa Core Microkernel ]  <-->  [ llama-microkernel ]  |  |
|  |          |                          (llama.cpp)         |  |
|  |          v                                              |  |
|  |  [ Internal Microservices & Modules ]                   |  |
|  |    - Tools / Services                                   |  |
|  |    - Micro-kernels (incl. optional env kernels)         |  |
|  |    - Binaries / Packages / Libraries                    |  |
|  |    - Agents (EOM / TSM / PSM / others)                  |  |
|  |                                                         |  |
|  |  [ App Hypervisor Layer ]                               |  |
|  |    - Hosted desktop/mobile apps                         |  |
|  |    - Shared or dedicated env per app                    |  |
|  |                                                         |  |
|  |  [ Sensor & Perception Pipeline ]                       |  |
|  |    - Mic / Camera / Screen / Activity                   |  |
|  |                                                         |  |
|  |  [ Persistence & Memory Layer ]                         |  |
|  +---------------------------------------------------------+  |
|                                                                |
+----------------------------------------------------------------+
       |                      |                       |
       v                      v                       v
  [ User Hive Fabric ]   [ Resource Mesh ]     [ Global Network ]
  (per-user P2P overlay) (per-user cluster)    (multi-user models)
```

---

## 5. Core Components

1. **Host Adapter Layer** – wraps OS/hardware APIs into a capability API.
2. **noa Core Microkernel** – process model, message bus, resource & policy engine.
3. **llama-microkernel (llama.cpp engine)** – inference microkernel managing small models.
4. **Internal Microservices & Modules** – tools, services, binaries, micro-kernels, agents.
5. **User Hive Fabric (UHF)** – per-user overlay network + shared memory across devices.
6. **Resource Mesh Layer** – cross-device scheduling and resource pooling.
7. **App Hypervisor Layer** – app hosting, isolation, data routing.
8. **Sensor & Perception Pipeline** – mic/camera/etc. → structured events.
9. **Persistence & Memory Layer** – long-term memory, logs, configs, model states.

---

## 6. Host Adapter Layer

### 6.1 Responsibilities

One adapter implementation per platform:

- `adapter-linux`, `adapter-windows11`, `adapter-macos`, `adapter-android`, `adapter-ios`, `adapter-xr`, etc.

Adapters:

- Wrap **host kernel and system services**:
  - Filesystem, processes, network, GPU, UI/windowing, notifications, sensors, timers.
- Expose a **capability API** to noa core:

Examples:

```ts
cap.fs.read(path, options) -> bytes
cap.fs.write(path, bytes, policy) -> result
cap.net.request(method, url, body, options) -> response
cap.ui.open_window(config) -> window_handle
cap.ui.render(component_tree, region) -> handle
cap.sensor.stream("mic" | "camera", options) -> stream_handle
cap.gpu.compute(kernel_desc, tensors, options) -> tensors
cap.timer.schedule(task_id, at | after) -> scheduled_job
```

### 6.2 Capability Description

Adapters expose their capabilities as a **declarative descriptor**:

```json
{
  "kernel": "linux",
  "device_id": "host-123",
  "cpu": {"cores": 8, "arch": "x86_64"},
  "gpu": {"type": "nvidia", "features": ["cuda", "tensor_cores"]},
  "ram_gb": 16,
  "ui": ["windows", "tray", "notifications"],
  "storage": ["ssd", "encrypted_home"],
  "sensors": ["camera", "microphone"],
  "extras": ["system-wide_shortcuts"]
}
```

noa core and the llama-microkernel reason about **capabilities**, not OS-specific APIs.

---

## 7. noa Core Microkernel

### 7.1 Role

The noa Core Microkernel is a **user-space kernel** responsible for:

- Process & task model for:
  - Agents
  - Tools
  - Services
  - Microkernels (incl. llama-microkernel as a special one)
- Message bus and event routing.
- Capability resolution & routing.
- Resource governance (CPU, GPU, memory, I/O, network, battery).
- Policy enforcement (security, privacy, cost, safety).
- Module lifecycle:
  - Install, activate, deactivate, rollback.
- Telemetry, logging, and audit.

### 7.2 Process Model

Each running entity is a **logical process**:

```ts
Process {
  id: UUID
  type: "agent" | "tool" | "service" | "microkernel" | "hosted_app"
  env_id: EnvID
  capabilities: CapabilitySet
  state: "running" | "paused" | "stopped" | "failed"
  resources: ResourceLimits
  version: SemanticVersion
}
```

Communication via **messages**:

```ts
Message {
  id: UUID
  from: ProcessID
  to: ProcessID | Topic
  type: "REQUEST" | "RESPONSE" | "EVENT"
  payload: JSON | binary
}
```

The microkernel includes a:

- **Scheduler**
- **Router**
- **Resource Governor**
- **Policy Gate** (calls into PSM when needed)

---

## 8. llama-microkernel (llama.cpp Engine)

### 8.1 Role

The **llama-microkernel** is a dedicated microkernel within noa, backed by **llama.cpp** (often mis-typed as llama.ccp). It:

- Manages all **local models** (text, code, small vision/audio if used).
- Provides a unified **inference API** to other components.
- Abstracts:
  - Model loading/unloading.
  - Quantization variants.
  - Device placement (CPU vs GPU).
  - Batching, caching, and scheduling.

### 8.2 Responsibilities

- Maintain registry of models:

```ts
Model {
  id: string
  size_params: number // <3B for core models
  type: "text" | "code" | "vision" | "audio" | "multi-modal"
  quantization: "q4" | "q5" | "q8" | ...
  location: "local" | "remote" | "hybrid"
  capabilities: string[]
}
```

- Expose inference calls:

```ts
llm.run({
  model_id,
  prompt,
  tools?,          // function-calling / tool schemas
  context_state?,  // previous tokens / KV cache
  max_tokens?,
  temperature?,
  system_instructions?
}) -> { output_text, new_context_state, tool_calls? }
```

- Host the **3 core small models (<3B)**:
  - **EOM** – Env Orchestrator Model.
  - **TSM** – Tool & Code Synthesizer Model.
  - **PSM** – Policy & Safety Model.

- Maintain **agent configurations** on top of these models:
  - Agents are prompt+tool+policy configurations.
  - Agents can be **spliced/merged/updated** over time.

### 8.3 Dynamic Behavior

- llama-microkernel is a **first-class microkernel module**:
  - It can be upgraded independently.
  - It can use device-specific optimizations from the Host Adapter.
  - It can share model weights across multiple agents and processes.

---

## 9. Core Small Models (EOM, TSM, PSM)

### 9.1 Env Orchestrator Model (EOM)

- Uses metrics, environment graphs, device capabilities, and goals to:
  - Propose environment changes.
  - Choose module versions.
  - Decide cross-device placement (via Resource Mesh Layer).
  - Mutate agent graphs (splicing/merging agents).

### 9.2 Tool & Code Synthesizer Model (TSM)

- Generates:
  - New tools.
  - Pipelines/DAGs.
  - Microservice code (within strict boundaries).
- Works in tandem with:
  - Build/test pipeline.
  - Static analysis.
  - PSM gating.

### 9.3 Policy & Safety Model (PSM)

- Evaluates proposals from EOM/TSM:
  - `ALLOW` / `DENY` / `REQUIRE_HUMAN_REVIEW`.
  - Risk scores, compliance reasoning.
- Enforces:
  - Data boundaries.
  - Sensitive capability use (e.g., camera/mic, off-device transfer).
  - Global + user-specific policies.

Together, these three sub-3B models **run noa’s brain** autonomously.

---

## 10. Modules & Internal Microservices

### 10.1 Module Definition

Every artifact (microkernel, binary, package, library, tool, service) is a **Module**:

```ts
Module {
  id: UUID
  name: string
  type: "microkernel" | "binary" | "package" | "library" | "tool" | "service"
  version: SemanticVersion
  hash: ContentHash
  dependencies: ModuleID[]
  capabilities_provided: CapabilityDescriptor[]
  capabilities_required: CapabilityDescriptor[]
  resources_required: ResourceDescriptor
  signatures: Signature[]
  metadata: JSON
}
```

Modules stored in a **content-addressable store (CAS)**:

- Immutable blobs, keyed by `hash`.
- New versions appended; old ones remain accessible.

### 10.2 Lifecycle

1. **Proposal** – EOM proposes new/updated/removed modules.
2. **Synthesis** – TSM generates new modules or selects existing ones.
3. **Validation** – build, tests, static analysis, PSM checks.
4. **Activation** – noa core loads module into an env.
5. **Rollout** – canary traffic, metrics collection.
6. **Promotion/Rollback** – based on telemetry.

Internally, these modules form **microservices** that communicate via the message bus.

---

## 11. User Hive Fabric (UHF)

### 11.1 Role

The UHF provides a **per-user P2P/overlay network**:

- Connects all noa instances for the same `$user`.
- Maintains a **User Hive Graph**:
  - Long-term memories.
  - Preferences.
  - High-level embeddings and knowledge.
- Supports:
  - State sync across devices.
  - Cross-device context & personalization.

### 11.2 Memory Model

- Use **event-sourced memory** or CRDT-like logs:

```ts
MemoryEvent {
  id: UUID
  user_id: UserID
  source_device: DeviceID
  time: Timestamp
  type: string
  payload_ref: ContentHash
  tags: string[]
  visibility: "LOCAL_ONLY" | "USER_SHARED" | "GLOBAL_OPT_IN"
}
```

- Heavy raw data (audio/video/files) is:
  - Processed locally (transcripts, embeddings).
  - Only semantic/derived events are usually shared across devices.

### 11.3 Sync & Consistency

- Devices sync via:
  - Direct connections when possible.
  - Relays otherwise.
- Conflict resolution:
  - CRDT or last-writer-wins with causal metadata.
- Policies:
  - Some events are never replicated (LOCAL_ONLY).
  - Some are shared across all devices (USER_SHARED).

---

## 12. Resource Mesh Layer

### 12.1 Per-User Cluster

All `$user` devices form a **Resource Mesh**:

- Each device publishes a **Resource Profile**:

```json
{
  "device_id": "laptop-1",
  "class": "laptop",
  "cpu": {"cores": 8},
  "gpu": {"type": "nvidia", "vram_gb": 8},
  "ram_gb": 16,
  "storage_free_gb": 200,
  "net": {"bandwidth": "50Mbps", "latency_ms": 30},
  "availability": "always_on",
  "cost_profile": "unconstrained"
}
```

- EOM + noa core decide **where** to run tasks, based on:
  - Resource fit.
  - Policy (battery-sensitive, privacy constraints).
  - Latency constraints.

### 12.2 Cross-Device Tasks

- A module on one device can request **remote execution**:

```ts
remote.execute({
  task: "embed_documents",
  payload_ref: ContentHash,
  constraints: {
    gpu_required: true,
    min_ram_gb: 8,
    on_device_class: ["desktop", "server"]
  }
}) -> result_ref
```

- Remote device executes in a sandboxed env and returns results.

### 12.3 Donating Resources to Global Network

- Each device has **donation policies**:
  - Time windows.
  - CPU/GPU caps.
  - Allowed task categories (training, inference, storage).
- Global Job Orchestrator:
  - Schedules global jobs onto volunteered devices.
- All global jobs:
  - Run in strictly sandboxed microkernels.
  - Are cryptographically signed and attestable.

---

## 13. App Hypervisor Layer (noa Hosting Other Apps)

### 13.1 Goals

- Give noa **VM-like control** without VM UX.
- Let noa **host/launch apps** so their data/traffic can be governed.
- Some apps:
  - Use **shared envs** (multi-tenant).
- Others:
  - Get **dedicated envs** (strong isolation).

### 13.2 Hosting Modes

1. **Shared Environment Mode**
   - Multiple hosted apps run in the same logical env.
   - Share:
     - Some tools/services.
     - Caches and models.
   - Have:
     - Per-app policies and namespaces where needed.

2. **Dedicated Environment Mode**
   - Hosted app runs in its own env:
     - Separate process set.
     - Separate filesystem view (via FUSE / virtual FS / OS sandbox).
     - Dedicated network policies.

### 13.3 Implementation Sketch (Desktop/Server)

- noa acts as:
  - **Launcher**:
    - Starts processes with specific sandbox settings (namespaces, cgroups, AppContainer, etc.).
  - **Network gateway**:
    - Local proxy / VPN to inspect & control app traffic.
  - **Filesystem mediator**:
    - Virtual FS overlays, per-app directories.

To the $user:

- Apps appear as normal windows.
- noa provides a unified launcher and management UI.

### 13.4 Mobile Limitations

- On stock Android/iOS:
  - noa cannot fully host other apps.
  - noa can:
    - Provide a VPN / proxy.
    - Offer share sheets / extensions for content.
    - Act as a context bridge where platform APIs allow.

---

## 14. Sensor & Perception Pipeline

### 14.1 Sensor Agents

Per-device **Sensor Agents**:

- `mic-agent`
- `camera-agent`
- Optional: `screen-agent`, `activity-agent`

Each:

- Requests OS permission explicitly.
- Has strict config:
  - On/off per sensor.
  - Sampling frequency.
  - Local only vs shareable.

### 14.2 Local Perception

Sensor data flows into a **Perception Pipeline**:

- Mic → on-device ASR (small model) → transcripts + semantic events.
- Camera → vision model → scene/face/gesture labels (no need for full images).
- Events are packaged as `MemoryEvent`s and fed into the User Hive.

Raw audio/video:

- Either:
  - Not stored at all.
  - Or kept in short temporal buffers (for debugging or explicit training modes).

### 14.3 Training Personal Hive Mind

- The User Hive uses perception events + app usage + text interactions to:
  - Build personalized embeddings.
  - Learn user preferences & patterns.
- EOM/TSM/PSM leverage this context to:
  - Improve planning.
  - Propose better agents & tools.
  - Customize behavior per `$user`.

### 14.4 Global Training with Consent

- Events have a **sharing classification**:
  - `LOCAL_ONLY`, `ANON_STATS_ONLY`, `SANITIZED_FEATURES`, `RAW_EXAMPLE`.
- A **Global Sync Agent**:
  - Subsamples & sanitizes shareable data.
  - Converts into:
    - Gradients / LoRA deltas.
    - Aggregated statistics.
- PSM ensures:
  - No obvious PII/secrets.
  - Policies are respected.

---

## 15. Persistence & Memory Layer

### 15.1 Static-ish Components

- **Core policies & constraints**  
- **Protocols & schemas** (capability schema, message formats).  
- **Identity & trust roots** (keys, certificates).  
- **Long-term audit logs** and backups.

These evolve slowly and with strict controls.

### 15.2 Dynamic Components

- Modules & versions.
- Env definitions and assignments.
- Agent configurations (including splicing/merging).
- Caches and intermediate artifacts.

All designed to be **frequently updated** and **revertible**.

---

## 16. Security, Privacy, and Policy

- Policies are enforced at multiple layers:
  - Host Adapter (OS constraints).
  - noa Core Microkernel (capabilities, resources).
  - PSM model (semantic policy reasoning).
- Important aspects:
  - Mic/camera are **never silently enabled**.
  - Off-device data transfer is policy-governed.
  - Sandbox boundaries for:
    - Hosted apps.
    - Global donation workloads.
    - Synthesized code (TSM output).

---

## 17. Deployment Profiles

### 17.1 Desktop (Linux, Windows 11, macOS)

- noa as:
  - Background service + tray/menu UI, or
  - System service + UI front-end.
- Strong app hosting and sandboxing opportunities.
- Best platform for heavy local models and background jobs.

### 17.2 Servers

- Same noa stack.
- Focus on:
  - High concurrency.
  - Acting as:
    - User resource backbone (for that user’s mesh).
    - Optional node in global model training.

### 17.3 Mobile / Tablets / Glasses

- noa as:
  - Standard app with:
    - Background services where allowed.
    - Notification integration.
- Limited app hosting; stronger focus on:
  - Perception (mic/camera).
  - Edge inference.
  - Periodic sync with other devices.

---

## 18. Human Interaction Model

Humans are **not in the operational loop**, but they:

- Define **goals**:
  - “Summarize my meetings and draft follow-ups.”
  - “Index and search all my documents.”
- Set **policies/constraints**:
  - Data locality.
  - Sensor usage.
  - Resource limits.
- Provide **feedback**:
  - Ratings.
  - Corrections.
  - High-level preferences.

These become:

- Hard constraints enforced by noa core + PSM.
- Reward signals for EOM/TSM’s self-improvement loops.

---

## 19. Summary

noa is:

- A **monolithic app with a microservice-style internal architecture**.
- A **host-kernel-based agentic runtime** that:
  - Spans all of a user’s devices as a **hive mind**.
  - Pools device resources into a **per-user cluster**.
  - Optionally contributes to a **global network**.
- A **hybrid VM/container host** for apps:
  - Independent envs and direct hardware access.
  - Native look & feel, no VM GUI.
- A **llama.cpp-driven brain**:
  - Via the **llama-microkernel** and 2–3 core <3B models (EOM, TSM, PSM).
  - With agents that continuously splice, merge, and evolve.

The system’s stability comes not from freezing anything, but from:

- Strong invariants (policies, contracts, schemas, identity).
- Strict, observable, reversible change pipelines.
- Constant adaptation guided by small, local models and real-world feedback from the $user’s entire device fleet.
