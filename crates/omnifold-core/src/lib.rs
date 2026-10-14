//! omnifold-core — executable specification and capability-graph verifier for
//! the **8B.is Omnifold (`ayePhone`)** cybernetic terminal.
//!
//! Bridges the five pillars of the Omnifold Beta ecosystem:
//! 1. [`capability`] — Vaked capability graph & `mem8` hash-verified continuity ledger.
//! 2. [`dma_ring`] — Zero-allocation fixed-slot DMA ring buffer feeding `CrabCC`
//!    context and sensory frames into Apple Silicon Unified Memory.
//! 3. [`bio_harmonic`] — 8B.is Bio-Harmonic Input Engine, Tri-Fold posture
//!    controller, and Teenage Engineering / R1 hybrid piezo rotary detents.
//! 4. [`universal_io`] — Raspberry Pi 40-pin HAT & Universal Plug/IO (`R-PI EXT v1.0`)
//!    controller with I3C hot-plug arbitration and 1.8V ↔ 3.3V level shifters.
//! 5. [`zeno_wallet`] — Embedded Zeno autonomous financial layer, Kintsugi
//!    double-entry ledger, and Sanpo Yoshi family/crew split (`*` Mom, `△` Dad).

pub mod bio_harmonic;
pub mod capability;
pub mod dma_ring;
pub mod universal_io;
pub mod zeno_wallet;

/// Official 5-color identity palette of the 8B.is Omnifold Beta System.
pub const PALETTE_HEX: [(&str, &str, &str); 5] = [
    (
        "LUMINANCE ORANGE",
        "#FA5B1C",
        "Primary hardware chassis & R-Pi extender",
    ),
    (
        "TERRITORY TEAL",
        "#00E5C9",
        "Micro-LiDAR sensors & volumetric light-field UI",
    ),
    (
        "GOLDEN RECONCILE",
        "#FFD700",
        "ZENO Kintsugi financial ledger & capability seals",
    ),
    (
        "CYBERNETIC GREEN",
        "#00FF66",
        "ayeOS, 8B.is Engine, and entheai quantum worktrees",
    ),
    (
        "OS VOID",
        "#0F1219",
        "UTG backdrop, Ghostty terminal void & titanium frame",
    ),
];
