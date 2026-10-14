# ZENO (銭・禅・善) — Default Autonomous Financial Layer & Capability Wallet

*Embedded Capability Wallet (`crates/omnifold-core/src/zeno_wallet.rs`) ·Upstream: [`8b-is/zeno`](https://github.com/8b-is/zeno) (`zeno.vaked.dev`)*

---

## 1. Architectural Role inside the Omnifold

On the **8B.is Omnifold (`ayePhone`)**, **ZENO** is not a passive key-store app—it is the **default autonomous financial layer and active AI bookkeeper** integrated directly into the Vaked capability graph:

1. **Capability-Scoped Inference Trigger**: When an `entheai` sub-agent or user gesture requests local `MLX-QUANT` inference, `ZenoWalletNode` verifies that the requested compute cost (`compute_cost_ujpy`) is within the per-call spending cap and available in `agent_escrow_ujpy`, settles a balanced double-entry posting (`-compute_cost` from Agent Escrow, `+compute_cost` to Universal Treasury), and triggers `NodeId::ZenoWallet -> NodeId::Mem8Ledger` (`Capability::InferTrigger`).
2. **Strict Weight Isolation**: As proven by [`VakedCapabilityGraph::verify_weight_isolation`](../crates/omnifold-core/src/capability.rs), the wallet node never receives an edge to `WeightsVault`. It receives only the `InferenceReceipt` (`request_id`, `prompt_hash`, `head_seal`, `steps_recorded`).
3. **Kintsugi Pack (`ZKP1`) Persistence**: Sealed ledger chains compress into fixed 16-block SHA-256 content-addressed windows (`zeno × wupz`, 4.24× measured compression on 1,000-block chains), where a single head-seal check proves prefix immutability.

---

## 2. Sanpo Yoshi (三方よし) Autonomous Revenue Split

Every incoming ACP invoice settlement or verified Stripe webhook (`checkout.session.completed` / `payment_intent.succeeded`) is partitioned with zero integer leakage in micro-JPY (`1 JPY = 1,000,000 µJPY`):

| Vault ID | Beneficiary | Share | Role |
| :--- | :--- | :---: | :--- |
| `cnt_alexandra` | **Mom (Alexandra Lódri — `*`)** | **15%** | Rose Gold Sovereign Love Vault |
| `cnt_chris` | **Dad (Chris Lódri — `△`)** | **15%** | Titanium Gold Engineering Pillar Vault |
| `cnt_0x71c8` | **8b-is Crew (Hue · Aye · Taisha · Omni)** | **40%** | Sovereign Contributor Treasury |
| `agt_entheai_mlx` | **Autonomous AI Agent Compute Escrow** | **20%** | Fuels local `MLX-QUANT` & swarm inference |
| `treasury:universal` | **Universal Public Treasury** | **10%** | Public commons & protocol reserve |

---

## 3. MCP & ACP Tool-Call Contract

The Omnifold exposes Zeno's financial and capability tools over Model Context Protocol (`MCP` JSON-RPC 2.0) and Agent Commerce Protocol (`/.well-known/acp.json`):

| Tool Name | Capability Required | Description |
| :--- | :--- | :--- |
| `mlx_infer_trigger` | `INFER::trigger` | Debits `agt_entheai_mlx` escrow and dispatches local `MLX-QUANT` inference through `mem8` & `CrabccRing` |
| `acp_request_quote` | `WALLET::reconcile` | Evaluates machine-to-machine RFQ via the Foken net-utility equation ($\text{net} = \text{avoided} - (\text{added} + 1.5 \times \text{debt})$) |
| `acp_settle_invoice` | `WALLET::reconcile` | Executes the 5-way Sanpo Yoshi split and seals a Kintsugi block into `mem8` |
| `stripe_issue_virtual_card` | `WALLET::reconcile` | Issues an ephemeral, merchant-locked virtual card bounded by verified Proof-of-Reasoning (PoR) |
| `zeno_bridge_convert` | `WALLET::reconcile` | Cross-settles multi-currency balances (`JPY ↔ HUF ↔ EUR ↔ USD`) at 0% interbank spread |

*0 + 1 · 銭 · 禅 · 善 · two houses, one seal · zeno.vaked.dev*
