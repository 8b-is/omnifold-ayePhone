//! ayephone — E2E verification & Ghostty/Kitty terminal HUD CLI for the
//! 8B.is Omnifold (`ayePhone`) cybernetic architecture.

use omnifold_core::bio_harmonic::{
    decode_sensory_frame, AirGesture, FoldPosture, RotaryDetentMode, SensoryFrame,
};
use omnifold_core::capability::{NodeId, VakedCapabilityGraph};
use omnifold_core::dma_ring::DmaRing;
use omnifold_core::universal_io::{ShifterDirection, UniversalIoBridge};
use omnifold_core::zeno_wallet::ZenoWalletNode;
use omnifold_core::PALETTE_HEX;

fn print_banner() {
    println!("┌─ 8 B . i s   O M N I F O L D   //   a y e P h o n e   v 2 . 0 . 0 - R C 1 ─┐");
    println!("│  S T A N D A R D   G A L A C T I C   ·   V A K E D   ·   M L X - Q U A N T │");
    println!("│  {{ - 1 ,   0 ,   + 1 }}   ·   m e m 8   ·   Z E N O   ·   e n t h e a i     │");
    println!("└────────────────────────────────────────────────────────────────────────────┘");
}

fn cmd_audit() {
    println!("\n▌ 1 .   Z E R O - C O S T   &   D M A   R I N G   A U D I T");
    let mut ring: DmaRing<8, 128> = DmaRing::new();
    let symbols = [
        b"crabcc::worktree::entheai_root" as &[u8],
        b"mlx_quant::ternary::qmm_t_fused",
        b"ayeos::memnet::linosv_seed",
        b"zeno::kintsugi::sanpo_yoshi_seal",
    ];
    for sym in symbols {
        let (seq, digest) = ring.feed(sym).expect("static DMA ring slot available");
        println!(
            "  ● DMA slot #{seq:02} <- {:<34} digest=0x{digest:016x}",
            std::str::from_utf8(sym).unwrap_or("")
        );
    }
    println!(
        "  ● Ring state: {}/{} slots occupied · zero heap allocations · rolling=0x{:016x}",
        ring.len(),
        ring.capacity(),
        ring.rolling_digest()
    );
}

fn cmd_graph() {
    println!("\n▌ 2 .   V A K E D   C A P A B I L I T Y   G R A P H   &   Z E N O   W A L L E T");
    let mut graph = VakedCapabilityGraph::omnifold_rc1();
    assert!(
        graph.verify_weight_isolation(),
        "weight isolation invariant must hold"
    );
    println!(
        "  ● Nodes: {} · Edges: {} · Weight isolation verified: {}",
        NodeId::ALL.len(),
        graph.edges().len(),
        graph.verify_weight_isolation()
    );

    let mut wallet = ZenoWalletNode::default();
    let split = wallet
        .settle_sanpo_yoshi(2026_1014, 10_000_000_000)
        .expect("positive gross settlement");
    println!(
        "  ● Sanpo Yoshi (10,000 JPY): Mom(*)={} µJPY · Dad(△)={} µJPY · Crew={} µJPY · Agent={} µJPY · Treasury={} µJPY",
        split.mom_alexandra_ujpy,
        split.dad_chris_ujpy,
        split.crew_8b_is_ujpy,
        split.agent_compute_ujpy,
        split.universal_treasury_ujpy
    );

    let receipt = wallet
        .authorize_and_trigger_inference(&mut graph, 1, 0x8b15_0001_cafe_babe, 5_000)
        .expect("authorized inference trigger");
    println!(
        "  ● Wallet -> mem8 -> CrabCC -> PolarQueue -> MLX-QUANT: {} edges verified · head_seal=0x{:016x}",
        receipt.steps_recorded, receipt.head_seal
    );
    println!("  ● mem8 continuity chain valid: {}", graph.verify_ledger());
}

fn cmd_simulate() {
    println!("\n▌ 3 .   B I O - H A R M O N I C   &   R - P I   U N I V E R S A L   I O");
    for posture in [
        FoldPosture::CompactPocket,
        FoldPosture::R1HybridZFold,
        FoldPosture::FullWorkbenchTriFold,
    ] {
        println!(
            "  ● Posture {:<20} -> UTG {:>3} Hz · Holo ceiling {:>3} mm · {:>5} spatial pts",
            format!("{posture:?}"),
            posture.utg_refresh_hz(),
            posture.max_holo_depth_mm(),
            posture.spatial_point_density()
        );
    }

    let mutation = decode_sensory_frame(SensoryFrame {
        posture: FoldPosture::FullWorkbenchTriFold,
        target_node: NodeId::EntheaiPortal,
        piezo_tendon_uv: 820,
        gesture: AirGesture::NodeFork { height_mm: 340 },
        acoustic_hz: 432,
    });
    println!(
        "  ● 432 Hz AirGesture::NodeFork -> op={:?} · depth={} mm · phase_locked={} · {} nits · hash=0x{:016x}",
        mutation.op,
        mutation.clamped_depth_mm,
        mutation.harmonic_phase_locked,
        mutation.luminescence_nits,
        mutation.event_hash
    );

    let (stiffness_mn, ticks) = RotaryDetentMode::CodeDiffScrub.profile();
    println!(
        "  ● R1 Rotary Wheel [CodeDiffScrub]: piezo detent={} mN · {} steps/rev",
        stiffness_mn, ticks
    );

    let mut graph = VakedCapabilityGraph::omnifold_rc1();
    let mut io = UniversalIoBridge::new();
    io.engage_mechanical_dock();
    let io_seal = io
        .arbitrate_hotplug(&mut graph, 0x40_9170)
        .expect("I3C hot-plug capability handshake");
    let dir = io
        .sample_or_drive_pin(11, ShifterDirection::Soc1v8ToHat3v3, 3_300)
        .expect("3.3V safe level shift");
    println!(
        "  ● R-PI EXT v1.0 40-pin HAT: I3C handshake seal=0x{io_seal:016x} · LHP shunt={} · Pin 11={dir:?}",
        io.is_lhp_shunt_active()
    );
}

fn cmd_palette() {
    println!("\n▌ 4 .   O M N I F O L D   B E T A   I D E N T I T Y   P A L E T T E");
    for (name, hex, role) in PALETTE_HEX {
        println!("  ■ {name:<18} {hex}  —  {role}");
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    print_banner();
    match arg.as_str() {
        "audit" => cmd_audit(),
        "graph" => cmd_graph(),
        "simulate" => cmd_simulate(),
        "palette" => cmd_palette(),
        _ => {
            cmd_audit();
            cmd_graph();
            cmd_simulate();
            cmd_palette();
        }
    }
    println!("\n*0 + 1 · folded back from infinity till forever · 8b.is / vaked.dev*");
}
