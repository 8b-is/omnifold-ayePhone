# RC1 · Capability Graph Mapping — 8B.is Omnifold (`ayePhone`)

*Deliverable 2 of 4 · System Directive: `ENTHEAI // MLX-QUANT RC1 INITIALIZATION` · v2.0.0-RC1 "Zeno"*

---

## 1. The 8-Node Omnifold Vaked Capability Graph

Implemented and verified in [`crates/omnifold-core/src/capability.rs`](../../crates/omnifold-core/src/capability.rs):

```text
  ┌──────────────────────────┐        ┌──────────────────────────┐
  │   ZenoWallet (wallet)    │        │  EntheaiPortal (agent)   │
  │  never reads weights     │        │ quantum worktree swarms  │
  └────────────┬─────────────┘        └────────────┬─────────────┘
               │ INFER::trigger                    │ INFER::trigger
               │ WALLET::reconcile                 │
               ▼                                   ▼
  ┌──────────────────────────────────────────────────────────────┐
  │              Mem8Ledger (ledger · hash-chained)              │
  │  · REQUEST::admit (self-check)                               │
  │  · routes CrabCC DMA ring & PolarQueue doorbell              │
  │  · arbitrates I3C hot-plug before energizing 3.3V R-Pi GPIO  │
  └─────┬──────────────────────┬───────────────────────┬─────────┘
        │ RING::feed           │ KERNEL::dispatch      │ GPIO::drive_3v3
        ▼                      ▼                       ▼
  ┌───────────────┐    ┌───────────────┐       ┌────────────────────┐
  │  CrabccRing   ├───>│  PolarQueue   │       │  RpiUniversalIo    │
  │ (DMA into UMA)│    │ (O(1) Metal)  │       │ (40-pin HAT + LHP) │
  └───────────────┘    └───┬───────┬───┘       └────────────────────┘
                           │       │ STATE::record (digest only)
             WEIGHTS::read │       └──────────────────┐
                           ▼                          ▼
                   ┌───────────────┐          ┌───────────────┐
                   │ WeightsVault  │          │  Mem8Ledger   │
                   │ {-1,0,+1} 2/4b│          │ (append-only) │
                   └───────────────┘          └───────────────┘
```

---

## 2. Canonical Vaked Node Configuration (TOML)

```toml
[node.zeno-wallet]
layer = "wallet"
grants = ["INFER::trigger", "WALLET::reconcile"]
isolation = "no outbound or inbound path to node.weights"

[node.entheai]
layer = "agent"
grants = ["INFER::trigger"]
worktree_engine = "CrabCC + parallel quantum worktrees"

[node.mem8]
layer = "ledger"
grants = ["REQUEST::admit", "RING::feed", "KERNEL::dispatch", "GPIO::drive_3v3"]
verify = "fnv1a64 + sha256 append-only seal chain"
genesis_seal = "0xcbf29ce484222325"

[node.crabcc-ring]
layer = "transport"
grants = ["KERNEL::dispatch"]
mode = "zero-allocation fixed-slot DMA ring into Unified Memory"
latency_budget = "sub-millisecond"

[node.polar-queue]
layer = "dispatch"
grants = ["WEIGHTS::read", "STATE::record"]
bypass = "O(1) Metal doorbell"

[node.weights]
layer = "params"
grants = [] # zero outbound edges
format = "ternary packed {-1, 0, +1} (12.80x) + mixed 2/3/4-bit"

[node.bio-harmonic]
layer = "sensory"
grants = ["GRAPH::mutate"]
sensors = ["Micro-LiDAR ToF", "Piezo Tendon Matrix", "432 Hz Acoustic Resonance"]

[node.rpi-universal-io]
layer = "hardware_io"
grants = ["I3C::handshake"]
require_before_3v3 = "I3C::handshake admitted in mem8"
```

---

## 3. Verified Security & Isolation Invariants

Pinned by unit tests in `crates/omnifold-core/src/capability.rs`:
1. **`weight_isolation_holds_and_wallet_cannot_read_weights`**:
   - `VakedCapabilityGraph::verify_weight_isolation()` confirms that **only** `NodeId::PolarQueue` holds `Capability::WeightsRead` on `NodeId::WeightsVault`, and `NodeId::WeightsVault` has zero outbound edges.
   - Any direct attempt by `ZenoWallet` or `EntheaiPortal` to read `WeightsVault` is rejected with `CapabilityRefusal::WeightIsolationBreach` and permanently logged in `mem8`.
2. **`wallet_triggered_inference_pipeline_succeeds_and_verifies`**:
   - Executes the 6-edge wallet-triggered inference sequence (`ZenoWallet -> Mem8Ledger -> CrabccRing -> PolarQueue -> WeightsVault` + `PolarQueue -> Mem8Ledger`), sealing `head_seal = 0xee7392ccf0adf6e7` and passing `verify_ledger()`.
3. **`gpio_drive_requires_i3c_hotplug_arbitration_first`**:
   - Attempting `GPIO::drive_3v3` before an admitted `I3C::handshake` fails with `CapabilityRefusal::HotplugHandshakeRequired`.

*graph closed · 8 nodes · 12 edges · 0 + 1 · 8b.is / vaked.dev*
