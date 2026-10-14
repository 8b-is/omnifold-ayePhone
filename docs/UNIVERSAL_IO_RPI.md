# 8B.is Universal Plug/IO & Raspberry Pi 40-Pin Module Extender (`R-PI EXT v1.0`)

*Hardware Subsystem Specification & Executable Controller Model (`crates/omnifold-core/src/universal_io.rs`)*

---

## 1. Overview & Mechanical Ergonomics

The **Raspberry Pi Compatible Module Extender (`R-PI EXT v1.0`)** snaps onto the bottom perimeter rail of the **8B.is Omnifold (`ayePhone`)**, transforming the tri-fold handheld into a hardware lab bench capable of driving standard 40-pin Raspberry Pi HATs, logic analyzers, SDR receivers, and I2C/SPI/I3C sensor arrays.

### Beta Mechanical Refinements
1. **Recessed Dust-Gasketed Spring-Pin Pogo Rails**: Replaces exposed landing strips with flush, gasket-sealed gold spring-pin arrays aligned by **Neodymium N52** rare-earth magnetic guides—eliminating pocket snag hazards and debris ingress.
2. **Low-Profile Dual Micro-Solenoid Slide-Cam Locks**: Flush perimeter cam latches engage once magnetic alignment is seated, preventing mechanical disconnect during high-vibration bench work.
3. **Active Inter-Segment Thermal Shunting**: Couples the Omnifold's internal micro-piezo Loop Heat Pipe (LHP) to the extender's thermal mass whenever power rails are energized.

---

## 2. Universal IO Bus Topology

```text
               +----------------------------------------+
               |     Apple Silicon SoC (PCIe Gen 4)     |
               +-------------------+--------------------+
                                   |
                                   v
             +---------------------------------------------+
             |    8B.is Universal IO High-Speed Bridge     |
             |       (FPGA / Low-Latency SerDes Hub)       |
             +------+--------------------+--------------+---+
                    |                    |              |
          +---------+--------+  +--------+-------+  +---+------------------+
          | Native RPi GPIO  |  | Low-Latency    |  | Dual-Lane MIPI CSI-2 |
          | Level Shifters   |  | I3C / SPI Bus  |  | / USB4 Retimer       |
          | (1.8V <-> 3.3V)  |  +----------------+  +----------------------+
          +---------+--------+
                    |
                    v
          +------------------+
          | Standard 40-Pin  |
          | HAT Header Array |
          +------------------+
```

- **Isolated 2-Lane PCIe Gen 4 SerDes Interconnect**: Routes directly from the SoC across the hinge to the extender's low-power FPGA/SerDes transceiver, guaranteeing high-bandwidth capture without serial bus bottlenecks.
- **Bidirectional 1.8V ↔ 3.3V Auto-Sensing Level Shifters**: Bridges 1.8V mobile silicon logic to standard 3.3V Raspberry Pi HAT logic on every GPIO pin.
- **Over-Voltage & Reverse-Polarity Hardware Clamps**: Trips into `HighImpedance` within nanoseconds if any header pin exceeds **3,600 mV (3.6V)** or drops below **-300 mV (-0.3V)**.

---

## 3. Smart Hot-Plug Capability Arbitration

To prevent voltage sags or unauthorized sub-agent hardware I/O from crashing running agent swarms, energizing the 40-pin header requires a strict 3-step sequence enforced by [`UniversalIoBridge`](../crates/omnifold-core/src/universal_io.rs) and [`VakedCapabilityGraph`](../crates/omnifold-core/src/capability.rs):

1. **Mechanical Dock Verification (`engage_mechanical_dock`)**: Confirms N52 magnetic alignment and dual micro-solenoid slide-cam lock engagement.
2. **I3C Sideband Capability Handshake (`I3C::handshake`)**: Reads the HAT's EEPROM/descriptor over dedicated I3C sideband pins and submits `NodeId::RpiUniversalIo -> NodeId::Mem8Ledger` (`Capability::I3cHandshake`).
3. **Ledger Authorization & Rail Energize (`GPIO::drive_3v3`)**: Only after `mem8` records the handshake seal does `Mem8Ledger -> RpiUniversalIo` grant `Capability::GpioDrive3v3`, energizing the 3.3V/5V rails and activating the Loop Heat Pipe thermal shunt.

```rust
let mut graph = VakedCapabilityGraph::omnifold_rc1();
let mut io = UniversalIoBridge::new();
io.engage_mechanical_dock();
let seal = io.arbitrate_hotplug(&mut graph, 0x40_9170)?;
let dir = io.sample_or_drive_pin(11, ShifterDirection::Soc1v8ToHat3v3, 3_300)?;
```

*0 + 1 · hardware capability arbitration · 8b.is / vaked.dev*
