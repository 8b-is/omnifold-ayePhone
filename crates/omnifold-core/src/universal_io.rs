//! universal_io — Raspberry Pi 40-pin HAT & Universal Plug/IO (`R-PI EXT v1.0`)
//! controller model.
//!
//! Implements the beta mechanical and electrical safeguards:
//! 1. Recessed dust-gasketed spring-pin pogo rails + Neodymium N52 magnetic
//!    alignment + dual micro-solenoid slide-cam locks.
//! 2. Microsecond I3C sideband hot-plug capability handshake before energizing
//!    power rails (preventing voltage sags or kernel panics across running
//!    agent swarms).
//! 3. Bidirectional 1.8V (SoC) ↔ 3.3V (R-Pi HAT) auto-sensing level shifters
//!    with over-voltage (> 3,600 mV) and reverse-polarity (< -300 mV) clamp
//!    protection on every header pin.
//! 4. Active micro-piezo Loop Heat Pipe (LHP) thermal shunting into the docked
//!    module backpack during heavy `MLX-QUANT` bursts.

use crate::capability::{Capability, CapabilityRefusal, NodeId, VakedCapabilityGraph};

/// Fault or refusal raised by the Universal IO controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoFault {
    /// Physical magnetic/pogo rail is not aligned and locked.
    NotMechanicallyLocked,
    /// Capability arbitration over I3C sideband has not completed.
    CapabilityHandshakeMissing,
    /// Invalid BCM/physical header pin index (must be 1..=40).
    InvalidHeaderPin(u8),
    /// Pin voltage exceeded 3,600 mV; hardware clamp tripped.
    OverVoltageClamp { pin: u8, mv: i32 },
    /// Pin voltage dropped below -300 mV; reverse-polarity protection tripped.
    ReversePolarityClamp { pin: u8, mv: i32 },
    /// Underlying Vaked capability refusal.
    Capability(CapabilityRefusal),
}

/// Direction auto-sensed by the 1.8V ↔ 3.3V level shifter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShifterDirection {
    HighImpedance,
    /// Apple Silicon SoC (1.8V) driving Raspberry Pi HAT (3.3V).
    Soc1v8ToHat3v3,
    /// Raspberry Pi HAT (3.3V) driving Apple Silicon SoC (1.8V).
    Hat3v3ToSoc1v8,
}

/// State of the Universal Plug/IO & Raspberry Pi 40-pin module extender.
#[derive(Debug, Clone)]
pub struct UniversalIoBridge {
    pogo_magnetic_aligned: bool,
    solenoid_cam_locked: bool,
    i3c_handshake_ok: bool,
    power_rails_energized: bool,
    lhp_shunt_active: bool,
    pin_directions: [ShifterDirection; 40],
    clamp_tripped: [bool; 40],
}

impl Default for UniversalIoBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalIoBridge {
    pub const fn new() -> Self {
        Self {
            pogo_magnetic_aligned: false,
            solenoid_cam_locked: false,
            i3c_handshake_ok: false,
            power_rails_energized: false,
            lhp_shunt_active: false,
            pin_directions: [ShifterDirection::HighImpedance; 40],
            clamp_tripped: [false; 40],
        }
    }

    pub const fn is_energized(&self) -> bool {
        self.power_rails_energized
    }

    pub const fn is_lhp_shunt_active(&self) -> bool {
        self.lhp_shunt_active
    }

    /// Step 1: seat module on N52 magnetic guide rails and engage dual
    /// micro-solenoid slide-cam locks.
    pub fn engage_mechanical_dock(&mut self) {
        self.pogo_magnetic_aligned = true;
        self.solenoid_cam_locked = true;
    }

    /// Step 2: run the I3C sideband capability handshake through `mem8` and,
    /// only upon verification, energize the 3.3V/5V HAT rails and Loop Heat Pipe.
    pub fn arbitrate_hotplug(
        &mut self,
        graph: &mut VakedCapabilityGraph,
        hat_descriptor_hash: u64,
    ) -> Result<u64, IoFault> {
        if !self.pogo_magnetic_aligned || !self.solenoid_cam_locked {
            return Err(IoFault::NotMechanicallyLocked);
        }
        graph
            .transit(
                NodeId::RpiUniversalIo,
                NodeId::Mem8Ledger,
                Capability::I3cHandshake,
                hat_descriptor_hash,
            )
            .map_err(IoFault::Capability)?;
        let seal = graph
            .transit(
                NodeId::Mem8Ledger,
                NodeId::RpiUniversalIo,
                Capability::GpioDrive3v3,
                hat_descriptor_hash,
            )
            .map_err(IoFault::Capability)?;

        self.i3c_handshake_ok = true;
        self.power_rails_energized = true;
        self.lhp_shunt_active = true;
        Ok(seal)
    }

    /// Configure and voltage-check one pin (1..=40) on the 40-pin HAT header.
    pub fn sample_or_drive_pin(
        &mut self,
        pin: u8,
        direction: ShifterDirection,
        external_mv: i32,
    ) -> Result<ShifterDirection, IoFault> {
        if !(1..=40).contains(&pin) {
            return Err(IoFault::InvalidHeaderPin(pin));
        }
        if !self.power_rails_energized || !self.i3c_handshake_ok {
            return Err(IoFault::CapabilityHandshakeMissing);
        }
        let idx = (pin - 1) as usize;
        if external_mv > 3_600 {
            self.clamp_tripped[idx] = true;
            self.pin_directions[idx] = ShifterDirection::HighImpedance;
            return Err(IoFault::OverVoltageClamp {
                pin,
                mv: external_mv,
            });
        }
        if external_mv < -300 {
            self.clamp_tripped[idx] = true;
            self.pin_directions[idx] = ShifterDirection::HighImpedance;
            return Err(IoFault::ReversePolarityClamp {
                pin,
                mv: external_mv,
            });
        }
        self.pin_directions[idx] = direction;
        Ok(direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotplug_sequence_and_overvoltage_clamp() {
        let mut g = VakedCapabilityGraph::omnifold_rc1();
        let mut io = UniversalIoBridge::new();

        // Cannot arbitrate before mechanical lock
        assert_eq!(
            io.arbitrate_hotplug(&mut g, 0xaaaa),
            Err(IoFault::NotMechanicallyLocked)
        );

        io.engage_mechanical_dock();
        assert!(io.arbitrate_hotplug(&mut g, 0xaaaa).is_ok());
        assert!(io.is_energized());
        assert!(io.is_lhp_shunt_active());

        // Valid 3.3V (3300 mV) GPIO operation on pin 7
        assert_eq!(
            io.sample_or_drive_pin(7, ShifterDirection::Soc1v8ToHat3v3, 3_300),
            Ok(ShifterDirection::Soc1v8ToHat3v3)
        );

        // 5.0V spike on a 3.3V GPIO pin trips the clamp immediately
        assert_eq!(
            io.sample_or_drive_pin(7, ShifterDirection::Hat3v3ToSoc1v8, 5_000),
            Err(IoFault::OverVoltageClamp { pin: 7, mv: 5_000 })
        );
    }
}
