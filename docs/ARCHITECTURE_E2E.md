# 8B.is Omnifold (`ayePhone`) — End-to-End System & Software Architecture

> *"A cybernetic terminal bridging multi-modal spatial computing, capability-graph security, sovereign agent swarms, and local ternary inference."*
>
> **Repositories**: [`github.com/8b-is`](https://github.com/8b-is) · [`github.com/standardgalactic`](https://github.com/standardgalactic) · [`github.com/peterlodri-sec`](https://github.com/peterlodri-sec)  
> **Crew**: Hue · Aye · Taisha · Omni · **Family**: Alexandra (`*`) · Chris (`△`) · Nate · Peter

---

## 1. System Overview & Full-Stack Topology

The **8B.is Omnifold (`ayePhone`)** unifies five foundational layers into a single tri-folding hardware and software architecture:

```text
+-----------------------------------------------------------------------+
|                         8B.is Omnifold Chassis                        |
|                                                                       |
|  +--------------------+  +--------------------+  +-----------------+  |
|  |   Tri-Fold Panel A |  |  Tri-Fold Panel B  |  | Tri-Fold Panel C|  |
|  |   (Volumetric      |  |  (Spatial Emitter  |  | (Primary UTG    |  |
|  |    Lightfield)     |  |   + Micro-LiDAR)   |  |  & Biometrics)  |  |
|  +--------------------+  +--------------------+  +-----------------+  |
|            |                        |                      |          |
|  +---------v------------------------v----------------------v-------+  |
|  |                  8B.is Bio-Harmonic Input Engine                |  |
|  |   (Spatial Gestures, Piezo Micro-Acoustics, 432 Hz Resonance)   |  |
|  +---------------------------------+-------------------------------+  |
|                                    |                                  |
|  +---------------------------------v-------------------------------+  |
|  |                    Application & Portal Layer                   |  |
|  |   - entheai (Sovereign Coding Agent & Quantum Worktrees)        |  |
|  |   - CrabCC (Rust Symbol Indexer & Zero-Copy MCP Context Feed)   |  |
|  |   - Volumetric Graph Visualizer & Ghostty/Kitty/Tmux Fallback   |  |
|  +---------------------------------+-------------------------------+  |
|                                    |                                  |
|  +---------------------------------v-------------------------------+  |
|  |               Vaked Runtime & Zeno Financial Layer              |  |
|  |   - Vaked Capability-Graph Resolution Engine                    |  |
|  |   - mem8 Zero-Allocation Hash-Verified Continuity Ledger        |  |
|  |   - Zeno Autonomous Capability Wallet & Sanpo Yoshi Split       |  |
|  +---------------------------------+-------------------------------+  |
|                                    |                                  |
|  +---------------------------------v-------------------------------+  |
|  |                     Execution & Inference Layer                 |  |
|  |   - 8B.is Engine (Concurrent Micro-Agent Orchestration Daemon)  |  |
|  |   - MLX-QUANT v2.0-RC1 "Zeno" (Ternary {-1,0,+1} & 2/3/4-bit)   |  |
|  +---------------------------------+-------------------------------+  |
|                                    |                                  |
|  +---------------------------------v-------------------------------+  |
|  |                       ayeOS (Core Operating System)             |  |
|  |  +----------------------------------+  +----------------------+ |  |
|  |  |   Mach-based Native Subsystem    |  | HarmonyOS Soft Bus   | |  |
|  |  +----------------------------------+  +----------------------+ |  |
|  |  +------------------------------------------------------------+ |  |
|  |  |  Unified Device Tree · Tailscale Aperture · Sandbox Ring   | |  |
|  |  +------------------------------------------------------------+ |  |
|  +---------------------------------+-------------------------------+  |
|                                    |                                  |
|  +---------------------------------v-------------------------------+  |
|  |   Apple Silicon Custom SoC (Unified Fabric) + R-Pi Universal IO |  |
|  +-----------------------------------------------------------------+  |
+-----------------------------------------------------------------------+
```

---

## 2. Core Software Stack (Rust · Zig · C)

The Omnifold runtime eliminates garbage-collected overhead on the critical path, executing across **Rust**, **Zig**, and **C**:

### 2.1 `ayeOS` — Foundational Operating System & Mesh Spine
- **Deterministic Ternary Seed**: Grounded in [`8b-is/ayeos`](https://github.com/8b-is/ayeos) (`SHA-256(LINOSV_SEED) -> Xoshiro128** PRNG -> ternary quantized weights`).
- **MEMNET Context Addressing**: Replaces static IP routing with intent- and context-addressed `.m8` knowledge capsules over `:9876`.
- **Hybrid Kernel Adaptation**: Bridges the Mach-based Apple Silicon native subsystem (`aarch64-apple-darwin`) with HarmonyOS Distributed Soft Bus protocols for cross-device edge continuity.
- **Honest Platform Sandbox Mapping**:
  - On **Linux / Android / HarmonyOS edge nodes**: kernel-level isolation uses Rust `aya` eBPF programs and `seccomp-bpf` syscall filters, plus hardware hypervisor memory maps for GPIO pin partitioning.
  - On **macOS / Darwin (`aarch64-apple-darwin`)**: where kernel eBPF/seccomp do not exist, isolation is enforced via macOS Seatbelt (`sandbox-exec`) profiles and mandatory userspace capability-graph mediation in [`omnifold-core::capability`](../crates/omnifold-core/src/capability.rs).

### 2.2 `8B.is Engine` — Concurrent Swarm Hyper-Visor
- **Admissible Continuation**: Built on [`8b-is/8b-is-engine`](https://github.com/8b-is/8b-is-engine) (`crates/admissibility`, `crates/spherepop`, `crates/ternary`, `crates/world-core`). Actions are admitted only when attested history guarantees safety across epistemically indistinguishable states.
- **Swarm Control Planes**: Natively schedules deployment templates from **AgentField**, **OpenFang**, and **NullClaw**.
- **Mesh Telemetry & Routing**: Task dispatch and state retention flow over **NATS JetStream** secured via **Tailscale Aperture**, binding external sub-agents to the Omnifold's root capability token.

### 2.3 `Vaked Runtime` & `mem8` Continuity Ledger
- **Capability Graph (`crates/omnifold-core/src/capability.rs`)**: System authority, Unified Memory windows, quantized model weights, and 40-pin Raspberry Pi HAT GPIOs are represented as explicit `NodeId` vertices connected by least-privilege `CapabilityEdge` grants.
- **Zero-Allocation Continuity Ledger (`mem8`)**: Every transition (both admitted calls and refused breaches) appends a deterministic `Mem8Entry` chained via `fnv1a_step(prev_seal, seq || from || to || capability || payload_hash || refusal_code)`.
- **Weight Isolation Invariant**: `ZenoWallet` and `EntheaiPortal` can trigger local inference (`INFER::trigger`), which routes through `Mem8Ledger` and `CrabccRing` to `PolarQueue`. Only `PolarQueue` holds `WEIGHTS::read` on `WeightsVault`, and `WeightsVault` has zero outbound edges—proving that neither the wallet nor an external agent can ever exfiltrate model weights.

### 2.4 `entheai` & `CrabCC` — Sovereign Coding Portal
- **Quantum Worktree Swarms**: `entheai` renders multi-repository git worktrees and literature as branching 3D spatial lattices above the unfolded Ultra-Thin Glass panels.
- **CrabCC Symbol Indexing (`crates/omnifold-core/src/dma_ring.rs`)**: `CrabCC` indexes Rust/Zig/C symbols locally and feeds context slices through a fixed-slot, zero-allocation `DmaRing<CAP, SLOT_BYTES>` directly into Unified Memory (`UMA`) with sub-millisecond latency and per-frame content digests.

### 2.5 `MLX-QUANT` v2.0.0-RC1 "Zeno" — Direct-to-Metal Ternary Executor
- **Fused Ternary & Mixed-Bit Kernels**: Executes BitNet b1.58 `{ -1, 0, +1 }` (12.80× compression vs fp32) and mixed 2-bit, 3-bit, and 4-bit weights directly on Apple Silicon Metal 4 GPU cores (`ternary_qmm_t` fused kernels beating compose fallbacks by 1.2–5.5×).
- **O(1) Polar Queue Doorbell**: Dispatched via static bump-heap memory and hardware doorbells without runtime JIT or Python interpreter overhead on the hot path.

### 2.6 `ZENO` (銭・禅・善) — Default Autonomous Financial Layer
- **Active AI Bookkeeper (`crates/omnifold-core/src/zeno_wallet.rs`)**: Embedded directly into the Vaked capability graph to manage scoped agent compute escrows, verify Proof-of-Reasoning (PoR) digests, and execute double-entry Kintsugi postings in micro-JPY (`µJPY`).
- **Sanpo Yoshi (三方よし) Autonomous Split**: Every settled invoice is split with zero leakage across Mom (`*` Alexandra, 15%), Dad (`△` Chris, 15%), 8b-is Crew (40%), Agent Compute Escrow (20%), and the Universal Public Treasury (10%).

---

## 3. Interaction Pipeline: Body & Environmental Modalities

```text
   Physical Input                   Extraction Pipeline                    Runtime Target
 [ Acoustic Pulse  ] ----> [ Piezo Transducer Matrix ] 
                                      |
                                      v
 [ Air Gesture     ] ----> [ Sub-mm LiDAR / Spatial  ] ---> [ 8B.is Harmonic Decoder ]
                                      |                            |
                                      v                            v
 [ Ambient Audio   ] ----> [ Acoustic Resonance MIC  ]    [ Vaked Graph Mutation Event ]
                                                                   |
                                                                   v
                                                      [ Holographic Node Transformation ]
```

Implemented in [`crates/omnifold-core/src/bio_harmonic.rs`](../crates/omnifold-core/src/bio_harmonic.rs):
1. **Acoustic Induction**: Samples environmental pitch and cadence; locking onto the **432 Hz ± 4 Hz** harmonic band boosts holographic luminescence up to the 3,200-nit ceiling.
2. **Micro-Kinetic Gesturing**: Sub-millimeter Micro-LiDAR arrays track `PinchExpand`, `DirectionalPull`, `NodeFork`, and `EdgeSever` gestures up to 450 mm above the UTG surface.
3. **Haptic Tactile Rotary Wheel**: The central hinge rotary wheel adapts its piezo-actuated force resistance (`110 mN` to `650 mN`) across `CodeDiffScrub`, `AudioFilterSweep`, `SdrFrequencyTune`, and `LedgerBlockStep`.

---

## 4. Interface Aesthetics & Terminal Fallback Layer

- **Volumetric Light-Field Typography**: Projects system telemetry, `mem8` hex-dumps, and capability nodes in a hybrid of **Standard Galactic glyphs** and high-contrast Japanese calligraphy inspired by 1995-era *Ghost in the Shell*.
- **Ghostty / Kitty / Tmux Fallback (`terminal/`)**: Dropping out of the volumetric light-field reveals a zero-distraction terminal cockpit rendered in **OS Void (`#0F1219`)**, **Cybernetic Green (`#00FF66`)**, **Luminance Orange (`#FA5B1C`)**, **Territory Teal (`#00E5C9`)**, and **Golden Reconcile (`#FFD700`)**.

*0 + 1 · the weave is the loop · 8b.is / vaked.dev*
