//! bio_harmonic — the 8B.is Bio-Harmonic Input Engine, Tri-Fold posture
//! controller, and Teenage Engineering / Rabbit r1 hybrid rotary detent model.
//!
//! Translates physical chassis posture, piezo tendon micro-vibrations,
//! sub-millimeter Micro-LiDAR air gestures, and ambient acoustic resonance
//! into deterministic Vaked capability-graph mutations and volumetric
//! holographic node transformations (up to 450 mm spatial depth above the UTG).

use crate::capability::{fnv1a_step, NodeId};

/// Physical tri-fold hinge posture of the 8B.is Omnifold chassis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldPosture {
    /// Folded pocket silhouette (low-power 24 Hz VRR, holographic array parked).
    CompactPocket,
    /// v1.0 R1 Hybrid Z-Fold (Panels A & B unfolded outward, Panel C inward;
    /// rotary scroll wheel & 360° ocular camera active).
    R1HybridZFold,
    /// Full 3-panel workbench unfold (120 Hz VRR, 3,200 nits, full 64K spatial
    /// light-field emitter cone at 160° and 450 mm depth).
    FullWorkbenchTriFold,
}

impl FoldPosture {
    /// Refresh rate in Hz for the Ultra-Thin Glass (UTG) substrate.
    pub const fn utg_refresh_hz(self) -> u32 {
        match self {
            FoldPosture::CompactPocket => 24,
            FoldPosture::R1HybridZFold => 90,
            FoldPosture::FullWorkbenchTriFold => 120,
        }
    }

    /// Maximum volumetric holographic projection ceiling in millimeters.
    pub const fn max_holo_depth_mm(self) -> u16 {
        match self {
            FoldPosture::CompactPocket => 0,
            FoldPosture::R1HybridZFold => 300,
            FoldPosture::FullWorkbenchTriFold => 450,
        }
    }

    /// Effective holographic spatial point density across 3D vector paths.
    pub const fn spatial_point_density(self) -> u32 {
        match self {
            FoldPosture::CompactPocket => 0,
            FoldPosture::R1HybridZFold => 32_768,
            FoldPosture::FullWorkbenchTriFold => 65_536, // 64K volumetric glyph layer
        }
    }
}

/// Programmatic piezo-actuated haptic detent profile for the central hinge
/// rotary scroll wheel (Teenage Engineering / R1 hybrid control).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotaryDetentMode {
    /// Crisp 24-step mechanical clicks for scrubbing `entheai` git worktree diffs.
    CodeDiffScrub,
    /// Smooth viscous resistance with center notch for 432 Hz audio filter sweeps.
    AudioFilterSweep,
    /// Fine 100-tick micro-detents for SDR / FLUG-OS radio frequency tuning.
    SdrFrequencyTune,
    /// Heavy golden notches for stepping through sealed Zeno Kintsugi blocks.
    LedgerBlockStep,
}

impl RotaryDetentMode {
    /// Piezo force resistance stiffness in milli-Newtons (mN) and steps per revolution.
    pub const fn profile(self) -> (u16, u16) {
        match self {
            RotaryDetentMode::CodeDiffScrub => (420, 24),
            RotaryDetentMode::AudioFilterSweep => (180, 12),
            RotaryDetentMode::SdrFrequencyTune => (110, 100),
            RotaryDetentMode::LedgerBlockStep => (650, 16),
        }
    }
}

/// Sub-millimeter Micro-LiDAR skeletal gesture above the chassis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AirGesture {
    /// Pinch and lift to expand a capability node into its sub-swarm view.
    PinchExpand { height_mm: u16 },
    /// Continuous directional pull to route context between nodes.
    DirectionalPull {
        dx_mm: i16,
        dy_mm: i16,
        height_mm: u16,
    },
    /// Two-finger split to fork an `entheai` quantum worktree node.
    NodeFork { height_mm: u16 },
    /// Sharp lateral slice to sever an active sub-agent capability edge.
    EdgeSever { height_mm: u16 },
}

impl AirGesture {
    pub const fn height_mm(self) -> u16 {
        match self {
            AirGesture::PinchExpand { height_mm }
            | AirGesture::DirectionalPull { height_mm, .. }
            | AirGesture::NodeFork { height_mm }
            | AirGesture::EdgeSever { height_mm } => height_mm,
        }
    }
}

/// Combined sensory frame sampled by the 8B.is Bio-Harmonic Input Engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SensoryFrame {
    pub posture: FoldPosture,
    pub target_node: NodeId,
    /// Piezoceramic tendon micro-vibration amplitude in micro-volts (µV).
    pub piezo_tendon_uv: u32,
    pub gesture: AirGesture,
    /// Ambient or intentional acoustic fundamental frequency in Hz (e.g. 432 Hz).
    pub acoustic_hz: u32,
}

/// Action emitted into the Vaked capability graph by the harmonic decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphMutationOp {
    ExpandNode,
    RouteContext,
    ForkWorktree,
    SeverCapabilityEdge,
    ParkedFolded,
}

/// Resulting holographic projection update & Vaked mutation receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HarmonicMutation {
    pub target_node: NodeId,
    pub op: GraphMutationOp,
    /// Clamped spatial projection height in millimeters (0..=450 mm).
    pub clamped_depth_mm: u16,
    /// True when `acoustic_hz` sits within the 432 Hz ± 4 Hz resonance band.
    pub harmonic_phase_locked: bool,
    /// Emit luminance in nits (up to 3,200 nits).
    pub luminescence_nits: u16,
    /// Deterministic event digest for `mem8` recording.
    pub event_hash: u64,
}

/// Decode a [`SensoryFrame`] into a deterministic [`HarmonicMutation`].
pub fn decode_sensory_frame(frame: SensoryFrame) -> HarmonicMutation {
    let max_depth = frame.posture.max_holo_depth_mm();
    if max_depth == 0 {
        let h = fnv1a_step(
            0xcbf2_9ce4_8422_2325,
            &[0, frame.target_node.slug().len() as u8],
        );
        return HarmonicMutation {
            target_node: frame.target_node,
            op: GraphMutationOp::ParkedFolded,
            clamped_depth_mm: 0,
            harmonic_phase_locked: false,
            luminescence_nits: 0,
            event_hash: h,
        };
    }

    let clamped_depth_mm = frame.gesture.height_mm().min(max_depth);
    let harmonic_phase_locked = (428..=436).contains(&frame.acoustic_hz);
    let op = match frame.gesture {
        AirGesture::PinchExpand { .. } => GraphMutationOp::ExpandNode,
        AirGesture::DirectionalPull { .. } => GraphMutationOp::RouteContext,
        AirGesture::NodeFork { .. } => GraphMutationOp::ForkWorktree,
        AirGesture::EdgeSever { .. } => GraphMutationOp::SeverCapabilityEdge,
    };

    let base_nits: u32 = 1_800 + (frame.piezo_tendon_uv.min(1_000));
    let bonus_nits: u32 = if harmonic_phase_locked { 400 } else { 0 };
    let luminescence_nits = (base_nits + bonus_nits).min(3_200) as u16;

    let mut h = fnv1a_step(0xcbf2_9ce4_8422_2325, frame.target_node.slug().as_bytes());
    h = fnv1a_step(h, &clamped_depth_mm.to_le_bytes());
    h = fnv1a_step(h, &frame.piezo_tendon_uv.to_le_bytes());
    h = fnv1a_step(h, &frame.acoustic_hz.to_le_bytes());

    HarmonicMutation {
        target_node: frame.target_node,
        op,
        clamped_depth_mm,
        harmonic_phase_locked,
        luminescence_nits,
        event_hash: h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unfolded_432hz_pinch_locks_phase_and_clamps_to_45cm() {
        let m = decode_sensory_frame(SensoryFrame {
            posture: FoldPosture::FullWorkbenchTriFold,
            target_node: NodeId::EntheaiPortal,
            piezo_tendon_uv: 850,
            gesture: AirGesture::NodeFork { height_mm: 520 },
            acoustic_hz: 432,
        });
        assert_eq!(m.op, GraphMutationOp::ForkWorktree);
        assert_eq!(m.clamped_depth_mm, 450);
        assert!(m.harmonic_phase_locked);
        assert_eq!(m.luminescence_nits, 3_050);
    }

    #[test]
    fn folded_pocket_parks_hologram() {
        let m = decode_sensory_frame(SensoryFrame {
            posture: FoldPosture::CompactPocket,
            target_node: NodeId::ZenoWallet,
            piezo_tendon_uv: 500,
            gesture: AirGesture::PinchExpand { height_mm: 200 },
            acoustic_hz: 432,
        });
        assert_eq!(m.op, GraphMutationOp::ParkedFolded);
        assert_eq!(m.clamped_depth_mm, 0);
        assert_eq!(m.luminescence_nits, 0);
    }
}
