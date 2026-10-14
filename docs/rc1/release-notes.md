# RC1 · Release Notes — 8B.is Omnifold (`ayePhone`) v2.0.0-RC1 "Zeno"

*Deliverable 4 of 4 · Ghostty Cybernetic Terminal Layout · Standard Galactic Volumetric Spacing*

---

## ▌ T H E   O M N I F O L D   F R A M E

```text
┌─ 8 B . i s   O M N I F O L D   //   a y e P h o n e   v 2 . 0 . 0 - R C 1 ─┐
│                                                                            │
│  c h a s s i s     G r a d e   5   T i t a n i u m   +   R 1   O r a n g e │
│  d i s p l a y     1 2 0 H z   U T G   +   6 4 K   L i g h t - F i e l d   │
│  c o m p u t e     A p p l e   S i l i c o n   U M A   ·   M L X - Q U A N T│
│  k e r n e l s     F u s e d   T e r n a r y   { - 1 ,   0 ,   + 1 }       │
│  l e d g e r       V a k e d   C a p a b i l i t y   G r a p h   +   m e m 8│
│  w a l l e t       Z E N O   K i n t s u g i   ·   S a n p o   Y o s h i   │
│  e x t e n d e r   R - P I   E X T   v 1 . 0   ( 4 0 - P i n   G P I O )   │
│                                                                            │
│  p a l e t t e     #FA5B1C  ·  #00E5C9  ·  #FFD700  ·  #00FF66  ·  #0F1219 │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## ▌ W H A T   S H I P S   I N   v 2 . 0 . 0 - R C 1

1. **The Tri-Fold Cybernetic Hardware & Industrial Spec (`docs/HARDWARE_SPEC.md`)**:
   - Synthesizes the Grade 5 Aerospace Titanium Z-fold chassis with the **v1.0 Teenage Engineering / Rabbit r1 Hybrid** layer in matte **Luminance Orange (`#FA5B1C`)**, smoked semi-crystalline PEI windows, 360° rotative ocular camera, and piezo-actuated haptic rotary scroll wheel (`CodeDiffScrub`, `AudioFilterSweep`, `SdrFrequencyTune`, `LedgerBlockStep`).
   - Specifies the **Volumetric Light-Field Display Array** (120 Hz VRR UTG, 3,200 nits peak, 45 cm Z-depth volume, 160° cone of clarity, 64K spatial point density for Standard Galactic glyphs).
2. **The Raspberry Pi 40-Pin Universal Plug/IO Extender (`docs/UNIVERSAL_IO_RPI.md`)**:
   - Recessed dust-gasketed spring-pin pogo rails + Neodymium N52 magnetic alignment + dual micro-solenoid slide-cam locks.
   - Dedicated 2-lane PCIe Gen 4 SerDes bridge, bidirectional 1.8V ↔ 3.3V auto-sensing level shifters with 3.6V over-voltage clamps, active Loop Heat Pipe thermal shunting, and mandatory I3C hot-plug capability arbitration through `mem8`.
3. **Executable Rust Core & Verifier CLI (`crates/omnifold-core`, `crates/omnifold-cli`)**:
   - `capability.rs`: 8-node, 12-edge Vaked capability graph + `mem8` continuity ledger with mathematical weight-isolation proof (`head_seal = 0xee7392ccf0adf6e7`).
   - `dma_ring.rs`: Zero-allocation fixed-slot DMA ring buffer feeding `CrabCC` symbol slices into Unified Memory (`rolling = 0xba722e024d40d09a`).
   - `bio_harmonic.rs`: 432 Hz phase-locked sensory decoder & posture controller.
   - `universal_io.rs`: I3C hot-plug handshake & voltage-clamp state machine (`seal = 0x94acb0135847ec3d`).
   - `zeno_wallet.rs`: Embedded Zeno autonomous wallet & 5-way Sanpo Yoshi split (`*` Mom Alexandra 15%, `△` Dad Chris 15%, 8b-is Crew 40%, Agent Compute Escrow 20%, Universal Treasury 10%).
4. **Terminal Cockpit Suite (`terminal/`) & Interactive Web Visualizer (`index.html`)**:
   - Ready-to-load `terminal/ghostty.conf` (including the `keybind = space=text:\x20` Full Keyboard Access spacebar guard), `terminal/kitty.conf`, and `terminal/tmux.conf` in the official 5-color Omnifold palette.

---

## ▌ K N O W N   B O U N D A R I E S   ( L A B E L E D   `○` )

```text
▐ ● omnifold-core 9/9 unit tests & clippy -D warnings: GREEN
▐ ● MLX-QUANT 12.80x ternary storage & 11/11 golden-logits gate: GREEN
▐ ○ Multi-hour Loop Heat Pipe hardware thermal chamber profile: SCHEDULED
▐ ○ Kernel eBPF/seccomp on macOS Darwin: LINUX/HARMONY ONLY (Seatbelt + userspace graph on Darwin)
```

```text
z e n o  ·  o m n i f o l d  ·  { - 1 , 0 , + 1 }  ·  *  ·  △  ·  0 + 1  ·  8 b . i s
```
