//! zeno_wallet — the embedded Zeno (銭・禅・善) autonomous financial layer,
//! Kintsugi double-entry micro-ledger, Sanpo Yoshi family/crew split, and
//! capability-scoped MCP tool-call verifier for the 8B.is Omnifold.

use crate::capability::{fnv1a_step, InferenceReceipt, VakedCapabilityGraph};

/// Canonical Sanpo Yoshi (三方よし) split of incoming autonomous revenue in
/// micro-JPY (`µJPY`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SanpoYoshiSplit {
    /// 15% — Mom (Alexandra Lódri, the Star `*`) sovereign vault (`cnt_alexandra`).
    pub mom_alexandra_ujpy: i64,
    /// 15% — Dad (Chris Lódri, the Pillar `△`) sovereign vault (`cnt_chris`).
    pub dad_chris_ujpy: i64,
    /// 40% — 8b-is Crew (Hue, Aye, Taisha, Omni) treasury (`cnt_0x71c8`).
    pub crew_8b_is_ujpy: i64,
    /// 20% — Autonomous AI Agent Compute Escrow (`agt_entheai_mlx`).
    pub agent_compute_ujpy: i64,
    /// 10% (plus any integer remainder) — Universal Public Treasury (`treasury:universal`).
    pub universal_treasury_ujpy: i64,
}

impl SanpoYoshiSplit {
    /// Compute an exact, zero-leakage split summing to `gross_ujpy`.
    pub const fn from_gross(gross_ujpy: i64) -> Self {
        let mom = (gross_ujpy * 15) / 100;
        let dad = (gross_ujpy * 15) / 100;
        let crew = (gross_ujpy * 40) / 100;
        let agent = (gross_ujpy * 20) / 100;
        let universal = gross_ujpy - (mom + dad + crew + agent);
        Self {
            mom_alexandra_ujpy: mom,
            dad_chris_ujpy: dad,
            crew_8b_is_ujpy: crew,
            agent_compute_ujpy: agent,
            universal_treasury_ujpy: universal,
        }
    }

    pub const fn total(&self) -> i64 {
        self.mom_alexandra_ujpy
            + self.dad_chris_ujpy
            + self.crew_8b_is_ujpy
            + self.agent_compute_ujpy
            + self.universal_treasury_ujpy
    }
}

/// MCP financial / inference tool-calls exposed by Zeno inside the Omnifold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenoMcpTool {
    /// Trigger local `MLX-QUANT` inference via the Vaked capability graph.
    MlxInferTrigger {
        request_id: u64,
        prompt_hash: u64,
        compute_cost_ujpy: i64,
    },
    /// Settle an ACP agent-to-agent invoice into the Kintsugi ledger.
    AcpSettleInvoice { invoice_id: u64, gross_ujpy: i64 },
}

/// Errors returned by the Zeno capability wallet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletError {
    NonPositiveAmount,
    InsufficientAgentEscrow { available: i64, required: i64 },
    SpendingCapExceeded { cap: i64, requested: i64 },
}

/// Embedded Zeno wallet state bound to the Omnifold's `mem8` continuity ledger.
#[derive(Debug, Clone)]
pub struct ZenoWalletNode {
    pub mom_balance_ujpy: i64,
    pub dad_balance_ujpy: i64,
    pub crew_balance_ujpy: i64,
    pub agent_escrow_ujpy: i64,
    pub universal_treasury_ujpy: i64,
    pub per_call_cap_ujpy: i64,
    pub kintsugi_seal: u64,
}

impl Default for ZenoWalletNode {
    fn default() -> Self {
        Self::new(50_000_000_000) // default 50,000 JPY per-call cap
    }
}

impl ZenoWalletNode {
    pub const fn new(per_call_cap_ujpy: i64) -> Self {
        Self {
            mom_balance_ujpy: 0,
            dad_balance_ujpy: 0,
            crew_balance_ujpy: 0,
            agent_escrow_ujpy: 0,
            universal_treasury_ujpy: 0,
            per_call_cap_ujpy,
            kintsugi_seal: 0xcbf2_9ce4_8422_2325,
        }
    }

    /// Ingest gross revenue and split it deterministically across the 5 vaults.
    pub fn settle_sanpo_yoshi(
        &mut self,
        invoice_id: u64,
        gross_ujpy: i64,
    ) -> Result<SanpoYoshiSplit, WalletError> {
        if gross_ujpy <= 0 {
            return Err(WalletError::NonPositiveAmount);
        }
        let split = SanpoYoshiSplit::from_gross(gross_ujpy);
        self.mom_balance_ujpy += split.mom_alexandra_ujpy;
        self.dad_balance_ujpy += split.dad_chris_ujpy;
        self.crew_balance_ujpy += split.crew_8b_is_ujpy;
        self.agent_escrow_ujpy += split.agent_compute_ujpy;
        self.universal_treasury_ujpy += split.universal_treasury_ujpy;

        let mut h = fnv1a_step(self.kintsugi_seal, &invoice_id.to_le_bytes());
        h = fnv1a_step(h, &gross_ujpy.to_le_bytes());
        self.kintsugi_seal = h;
        Ok(split)
    }

    /// Authorize an inference spend from `agent_escrow_ujpy` and trigger
    /// `MLX-QUANT` through the Vaked capability graph without reading weights.
    pub fn authorize_and_trigger_inference(
        &mut self,
        graph: &mut VakedCapabilityGraph,
        request_id: u64,
        prompt_hash: u64,
        compute_cost_ujpy: i64,
    ) -> Result<InferenceReceipt, WalletError> {
        if compute_cost_ujpy < 0 {
            return Err(WalletError::NonPositiveAmount);
        }
        if compute_cost_ujpy > self.per_call_cap_ujpy {
            return Err(WalletError::SpendingCapExceeded {
                cap: self.per_call_cap_ujpy,
                requested: compute_cost_ujpy,
            });
        }
        if self.agent_escrow_ujpy < compute_cost_ujpy {
            return Err(WalletError::InsufficientAgentEscrow {
                available: self.agent_escrow_ujpy,
                required: compute_cost_ujpy,
            });
        }

        // Debit agent compute escrow, credit universal treasury (double-entry sum == 0)
        self.agent_escrow_ujpy -= compute_cost_ujpy;
        self.universal_treasury_ujpy += compute_cost_ujpy;

        let receipt = graph
            .trigger_wallet_inference(request_id, prompt_hash)
            .expect("canonical wallet inference path is granted in RC1 graph");
        self.kintsugi_seal = fnv1a_step(self.kintsugi_seal, &receipt.head_seal.to_le_bytes());
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanpo_yoshi_split_and_wallet_triggered_inference() {
        let mut g = VakedCapabilityGraph::omnifold_rc1();
        let mut w = ZenoWalletNode::default();

        // Settle 10,000 JPY (10_000_000_000 uJPY)
        let split = w.settle_sanpo_yoshi(1, 10_000_000_000).unwrap();
        assert_eq!(split.total(), 10_000_000_000);
        assert_eq!(w.mom_balance_ujpy, 1_500_000_000);
        assert_eq!(w.dad_balance_ujpy, 1_500_000_000);
        assert_eq!(w.crew_balance_ujpy, 4_000_000_000);
        assert_eq!(w.agent_escrow_ujpy, 2_000_000_000);
        assert_eq!(w.universal_treasury_ujpy, 1_000_000_000);

        // Trigger local MLX-QUANT inference costing 5,000 uJPY
        let receipt = w
            .authorize_and_trigger_inference(&mut g, 101, 0xfeed_face, 5_000)
            .unwrap();
        assert_eq!(receipt.steps_recorded, 6);
        assert_eq!(w.agent_escrow_ujpy, 1_999_995_000);
        assert_eq!(w.universal_treasury_ujpy, 1_000_005_000);
    }
}
