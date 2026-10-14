//! capability — the Vaked capability-graph engine and `mem8` hash-verified
//! continuity ledger for the 8B.is Omnifold (`ayePhone`).
//!
//! Authority, memory spaces, quantized weights, and hardware GPIO pins are
//! modeled as explicit graph nodes with least-privilege capability edges.
//! Every transition — whether admitted or refused — appends to the `mem8`
//! hash-chained continuity ledger.
//!
//! Core architectural invariant:
//! - `ZenoWallet` and `EntheaiPortal` can trigger local inference
//!   (`INFER::trigger`), which routes through `Mem8Ledger` to `CrabccRing` and
//!   `PolarQueue`.
//! - Neither `ZenoWallet` nor `EntheaiPortal` has any direct or transitive edge
//!   that returns `WEIGHTS::read`. Only `PolarQueue` reads weights from
//!   `WeightsVault` to execute fused ternary `{ -1, 0, +1 }` kernels.

use std::fmt;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Deterministic 64-bit FNV-1a fold used across `mem8` continuity entries.
pub fn fnv1a_step(mut state: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        state ^= u64::from(b);
        state = state.wrapping_mul(FNV_PRIME);
    }
    state
}

/// Canonical nodes in the 8B.is Omnifold Vaked capability graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeId {
    /// Autonomous financial layer & capability wallet (`zeno-wallet`).
    ZenoWallet,
    /// Zero-allocation hash-verified continuity ledger (`mem8`).
    Mem8Ledger,
    /// Zero-allocation DMA ring buffer feeding CrabCC symbol slices (`crabcc-ring`).
    CrabccRing,
    /// Read-only ternary `{ -1, 0, +1 }` and mixed 2/3/4-bit weight vault (`weights`).
    WeightsVault,
    /// O(1) doorbell Metal / co-processor dispatch queue (`polar-queue`).
    PolarQueue,
    /// Piezo, Micro-LiDAR & 432 Hz acoustic sensory decoder (`bio-harmonic`).
    BioHarmonic,
    /// Raspberry Pi 40-pin HAT & PCIe Gen 4 SerDes Universal IO bridge (`rpi-universal-io`).
    RpiUniversalIo,
    /// Sovereign coding agent & quantum worktree portal (`entheai`).
    EntheaiPortal,
}

impl NodeId {
    pub const ALL: [NodeId; 8] = [
        NodeId::ZenoWallet,
        NodeId::Mem8Ledger,
        NodeId::CrabccRing,
        NodeId::WeightsVault,
        NodeId::PolarQueue,
        NodeId::BioHarmonic,
        NodeId::RpiUniversalIo,
        NodeId::EntheaiPortal,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            NodeId::ZenoWallet => "zeno-wallet",
            NodeId::Mem8Ledger => "mem8",
            NodeId::CrabccRing => "crabcc-ring",
            NodeId::WeightsVault => "weights",
            NodeId::PolarQueue => "polar-queue",
            NodeId::BioHarmonic => "bio-harmonic",
            NodeId::RpiUniversalIo => "rpi-universal-io",
            NodeId::EntheaiPortal => "entheai",
        }
    }

    pub fn layer(self) -> &'static str {
        match self {
            NodeId::ZenoWallet => "wallet",
            NodeId::Mem8Ledger => "ledger",
            NodeId::CrabccRing => "transport",
            NodeId::WeightsVault => "params",
            NodeId::PolarQueue => "dispatch",
            NodeId::BioHarmonic => "sensory",
            NodeId::RpiUniversalIo => "hardware_io",
            NodeId::EntheaiPortal => "agent",
        }
    }

    fn tag(self) -> u8 {
        match self {
            NodeId::ZenoWallet => 1,
            NodeId::Mem8Ledger => 2,
            NodeId::CrabccRing => 3,
            NodeId::WeightsVault => 4,
            NodeId::PolarQueue => 5,
            NodeId::BioHarmonic => 6,
            NodeId::RpiUniversalIo => 7,
            NodeId::EntheaiPortal => 8,
        }
    }
}

/// Capabilities mediated by the Vaked graph router.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    InferTrigger,
    RequestAdmit,
    RingFeed,
    WeightsRead,
    KernelDispatch,
    StateRecord,
    GraphMutate,
    I3cHandshake,
    GpioDrive3v3,
    WalletReconcile,
}

impl Capability {
    pub fn name(self) -> &'static str {
        match self {
            Capability::InferTrigger => "INFER::trigger",
            Capability::RequestAdmit => "REQUEST::admit",
            Capability::RingFeed => "RING::feed",
            Capability::WeightsRead => "WEIGHTS::read",
            Capability::KernelDispatch => "KERNEL::dispatch",
            Capability::StateRecord => "STATE::record",
            Capability::GraphMutate => "GRAPH::mutate",
            Capability::I3cHandshake => "I3C::handshake",
            Capability::GpioDrive3v3 => "GPIO::drive_3v3",
            Capability::WalletReconcile => "WALLET::reconcile",
        }
    }

    fn tag(self) -> u8 {
        match self {
            Capability::InferTrigger => 1,
            Capability::RequestAdmit => 2,
            Capability::RingFeed => 3,
            Capability::WeightsRead => 4,
            Capability::KernelDispatch => 5,
            Capability::StateRecord => 6,
            Capability::GraphMutate => 7,
            Capability::I3cHandshake => 8,
            Capability::GpioDrive3v3 => 9,
            Capability::WalletReconcile => 10,
        }
    }
}

/// Directed capability edge in the Vaked graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub capability: Capability,
}

/// Why a capability transition was refused by the Vaked router.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityRefusal {
    /// No edge in the capability graph grants `(from -> to, capability)`.
    EdgeNotGranted,
    /// Direct weight read attempted by a non-dispatch node.
    WeightIsolationBreach,
    /// `GPIO::drive_3v3` attempted before an admitted `I3C::handshake`.
    HotplugHandshakeRequired,
}

impl fmt::Display for CapabilityRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CapabilityRefusal::EdgeNotGranted => write!(f, "edge not granted in Vaked graph"),
            CapabilityRefusal::WeightIsolationBreach => {
                write!(f, "weight isolation breach: caller may not read weights")
            }
            CapabilityRefusal::HotplugHandshakeRequired => {
                write!(
                    f,
                    "I3C hot-plug capability handshake required before driving 3.3V GPIO"
                )
            }
        }
    }
}

impl std::error::Error for CapabilityRefusal {}

/// One append-only record in the `mem8` continuity ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mem8Entry {
    pub seq: u64,
    pub from: NodeId,
    pub to: NodeId,
    pub capability: Capability,
    pub payload_hash: u64,
    pub refusal: Option<CapabilityRefusal>,
    pub prev_seal: u64,
    pub seal: u64,
}

/// Receipt emitted after a complete wallet-triggered local inference pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InferenceReceipt {
    pub request_id: u64,
    pub prompt_hash: u64,
    pub head_seal: u64,
    pub steps_recorded: usize,
}

/// The Vaked Capability Graph and `mem8` continuity ledger.
#[derive(Debug, Clone)]
pub struct VakedCapabilityGraph {
    edges: Vec<CapabilityEdge>,
    ledger: Vec<Mem8Entry>,
    i3c_arbitrated: bool,
}

impl Default for VakedCapabilityGraph {
    fn default() -> Self {
        Self::omnifold_rc1()
    }
}

impl VakedCapabilityGraph {
    pub const GENESIS_SEAL: u64 = FNV_OFFSET;

    /// Construct the canonical RC1 Omnifold capability graph.
    pub fn omnifold_rc1() -> Self {
        let edges = vec![
            // Wallet & agent trigger paths into mem8
            CapabilityEdge {
                from: NodeId::ZenoWallet,
                to: NodeId::Mem8Ledger,
                capability: Capability::InferTrigger,
            },
            CapabilityEdge {
                from: NodeId::ZenoWallet,
                to: NodeId::Mem8Ledger,
                capability: Capability::WalletReconcile,
            },
            CapabilityEdge {
                from: NodeId::EntheaiPortal,
                to: NodeId::Mem8Ledger,
                capability: Capability::InferTrigger,
            },
            // mem8 admission, DMA routing, dispatch, and state recording
            CapabilityEdge {
                from: NodeId::Mem8Ledger,
                to: NodeId::Mem8Ledger,
                capability: Capability::RequestAdmit,
            },
            CapabilityEdge {
                from: NodeId::Mem8Ledger,
                to: NodeId::CrabccRing,
                capability: Capability::RingFeed,
            },
            CapabilityEdge {
                from: NodeId::CrabccRing,
                to: NodeId::PolarQueue,
                capability: Capability::KernelDispatch,
            },
            CapabilityEdge {
                from: NodeId::Mem8Ledger,
                to: NodeId::PolarQueue,
                capability: Capability::KernelDispatch,
            },
            // Only PolarQueue reads weights from WeightsVault
            CapabilityEdge {
                from: NodeId::PolarQueue,
                to: NodeId::WeightsVault,
                capability: Capability::WeightsRead,
            },
            // PolarQueue records output digest back to mem8 (never raw weights)
            CapabilityEdge {
                from: NodeId::PolarQueue,
                to: NodeId::Mem8Ledger,
                capability: Capability::StateRecord,
            },
            // Bio-Harmonic sensory mutation through mem8
            CapabilityEdge {
                from: NodeId::BioHarmonic,
                to: NodeId::Mem8Ledger,
                capability: Capability::GraphMutate,
            },
            // Universal IO R-Pi 40-pin hot-plug arbitration
            CapabilityEdge {
                from: NodeId::RpiUniversalIo,
                to: NodeId::Mem8Ledger,
                capability: Capability::I3cHandshake,
            },
            CapabilityEdge {
                from: NodeId::Mem8Ledger,
                to: NodeId::RpiUniversalIo,
                capability: Capability::GpioDrive3v3,
            },
        ];
        Self {
            edges,
            ledger: Vec::new(),
            i3c_arbitrated: false,
        }
    }

    pub fn edges(&self) -> &[CapabilityEdge] {
        &self.edges
    }

    pub fn ledger(&self) -> &[Mem8Entry] {
        &self.ledger
    }

    pub fn head_seal(&self) -> u64 {
        self.ledger.last().map_or(Self::GENESIS_SEAL, |e| e.seal)
    }

    pub fn is_i3c_arbitrated(&self) -> bool {
        self.i3c_arbitrated
    }

    /// Check whether a direct edge `(from -> to, capability)` is present.
    pub fn has_edge(&self, from: NodeId, to: NodeId, capability: Capability) -> bool {
        self.edges
            .iter()
            .any(|e| e.from == from && e.to == to && e.capability == capability)
    }

    /// Verify the RC1 isolation rule: no node other than `PolarQueue` holds a
    /// `WEIGHTS::read` edge to `WeightsVault`, and `WeightsVault` has zero
    /// outbound edges.
    pub fn verify_weight_isolation(&self) -> bool {
        let only_polar_reads = self
            .edges
            .iter()
            .filter(|e| e.capability == Capability::WeightsRead || e.to == NodeId::WeightsVault)
            .all(|e| {
                e.from == NodeId::PolarQueue
                    && e.to == NodeId::WeightsVault
                    && e.capability == Capability::WeightsRead
            });
        let no_outbound_from_weights = self.edges.iter().all(|e| e.from != NodeId::WeightsVault);
        only_polar_reads && no_outbound_from_weights
    }

    fn compute_seal(
        prev_seal: u64,
        seq: u64,
        from: NodeId,
        to: NodeId,
        capability: Capability,
        payload_hash: u64,
        refusal: Option<CapabilityRefusal>,
    ) -> u64 {
        let mut h = fnv1a_step(prev_seal, &seq.to_le_bytes());
        h = fnv1a_step(h, &[from.tag(), to.tag(), capability.tag()]);
        h = fnv1a_step(h, &payload_hash.to_le_bytes());
        let refusal_code = match refusal {
            None => 0u8,
            Some(CapabilityRefusal::EdgeNotGranted) => 1,
            Some(CapabilityRefusal::WeightIsolationBreach) => 2,
            Some(CapabilityRefusal::HotplugHandshakeRequired) => 3,
        };
        fnv1a_step(h, &[refusal_code])
    }

    /// Attempt a capability transition, recording the outcome in `mem8`.
    pub fn transit(
        &mut self,
        from: NodeId,
        to: NodeId,
        capability: Capability,
        payload_hash: u64,
    ) -> Result<u64, CapabilityRefusal> {
        let check = if (capability == Capability::WeightsRead || to == NodeId::WeightsVault)
            && from != NodeId::PolarQueue
        {
            Err(CapabilityRefusal::WeightIsolationBreach)
        } else if capability == Capability::GpioDrive3v3 && !self.i3c_arbitrated {
            Err(CapabilityRefusal::HotplugHandshakeRequired)
        } else if !self.has_edge(from, to, capability) {
            Err(CapabilityRefusal::EdgeNotGranted)
        } else {
            Ok(())
        };

        let seq = self.ledger.len() as u64 + 1;
        let prev_seal = self.head_seal();
        let refusal = check.err();
        let seal = Self::compute_seal(prev_seal, seq, from, to, capability, payload_hash, refusal);
        self.ledger.push(Mem8Entry {
            seq,
            from,
            to,
            capability,
            payload_hash,
            refusal,
            prev_seal,
            seal,
        });

        match check {
            Ok(()) => {
                if capability == Capability::I3cHandshake {
                    self.i3c_arbitrated = true;
                }
                Ok(seal)
            }
            Err(err) => Err(err),
        }
    }

    /// Execute the canonical 5-step wallet-triggered inference flow:
    /// 1. `zeno-wallet -> mem8` (`INFER::trigger`)
    /// 2. `mem8 -> mem8` (`REQUEST::admit`)
    /// 3. `mem8 -> crabcc-ring` (`RING::feed`)
    /// 4. `crabcc-ring -> polar-queue` (`KERNEL::dispatch`)
    /// 5. `polar-queue -> weights` (`WEIGHTS::read`)
    /// 6. `polar-queue -> mem8` (`STATE::record`)
    pub fn trigger_wallet_inference(
        &mut self,
        request_id: u64,
        prompt_hash: u64,
    ) -> Result<InferenceReceipt, CapabilityRefusal> {
        let start_len = self.ledger.len();
        let mix = fnv1a_step(prompt_hash, &request_id.to_le_bytes());
        self.transit(
            NodeId::ZenoWallet,
            NodeId::Mem8Ledger,
            Capability::InferTrigger,
            mix,
        )?;
        self.transit(
            NodeId::Mem8Ledger,
            NodeId::Mem8Ledger,
            Capability::RequestAdmit,
            mix,
        )?;
        self.transit(
            NodeId::Mem8Ledger,
            NodeId::CrabccRing,
            Capability::RingFeed,
            mix,
        )?;
        self.transit(
            NodeId::CrabccRing,
            NodeId::PolarQueue,
            Capability::KernelDispatch,
            mix,
        )?;
        self.transit(
            NodeId::PolarQueue,
            NodeId::WeightsVault,
            Capability::WeightsRead,
            mix,
        )?;
        let head_seal = self.transit(
            NodeId::PolarQueue,
            NodeId::Mem8Ledger,
            Capability::StateRecord,
            mix,
        )?;
        Ok(InferenceReceipt {
            request_id,
            prompt_hash,
            head_seal,
            steps_recorded: self.ledger.len() - start_len,
        })
    }

    /// Recompute every `mem8` seal from genesis to verify ledger continuity.
    pub fn verify_ledger(&self) -> bool {
        let mut expected_prev = Self::GENESIS_SEAL;
        for (idx, e) in self.ledger.iter().enumerate() {
            if e.seq != idx as u64 + 1 || e.prev_seal != expected_prev {
                return false;
            }
            let expected_seal = Self::compute_seal(
                e.prev_seal,
                e.seq,
                e.from,
                e.to,
                e.capability,
                e.payload_hash,
                e.refusal,
            );
            if e.seal != expected_seal {
                return false;
            }
            expected_prev = e.seal;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_isolation_holds_and_wallet_cannot_read_weights() {
        let mut g = VakedCapabilityGraph::omnifold_rc1();
        assert!(g.verify_weight_isolation());

        let res = g.transit(
            NodeId::ZenoWallet,
            NodeId::WeightsVault,
            Capability::WeightsRead,
            0xdead_beef,
        );
        assert_eq!(res, Err(CapabilityRefusal::WeightIsolationBreach));
        assert_eq!(g.ledger().len(), 1);
        assert!(g.verify_ledger());
    }

    #[test]
    fn wallet_triggered_inference_pipeline_succeeds_and_verifies() {
        let mut g = VakedCapabilityGraph::omnifold_rc1();
        let receipt = g.trigger_wallet_inference(42, 0x1234_5678).unwrap();
        assert_eq!(receipt.steps_recorded, 6);
        assert_eq!(receipt.head_seal, g.head_seal());
        assert!(g.verify_ledger());
    }

    #[test]
    fn gpio_drive_requires_i3c_hotplug_arbitration_first() {
        let mut g = VakedCapabilityGraph::omnifold_rc1();
        assert_eq!(
            g.transit(
                NodeId::Mem8Ledger,
                NodeId::RpiUniversalIo,
                Capability::GpioDrive3v3,
                1
            ),
            Err(CapabilityRefusal::HotplugHandshakeRequired)
        );
        assert!(g
            .transit(
                NodeId::RpiUniversalIo,
                NodeId::Mem8Ledger,
                Capability::I3cHandshake,
                1
            )
            .is_ok());
        assert!(g
            .transit(
                NodeId::Mem8Ledger,
                NodeId::RpiUniversalIo,
                Capability::GpioDrive3v3,
                2
            )
            .is_ok());
        assert!(g.verify_ledger());
    }
}
