# RC1 · Build Matrix — `aarch64-apple-darwin` (M1 / M3 Pro & Max)

*Deliverable 3 of 4 · System Directive: `ENTHEAI // MLX-QUANT RC1 INITIALIZATION` · v2.0.0-RC1 "Zeno"*

---

## 1. Omnifold Core & CLI (`omnifold-ayePhone` Rust Workspace)

```bash
# Release build (opt-level = 3, fat LTO, codegen-units = 1, strip = true, panic = "abort")
cargo build --release --target aarch64-apple-darwin

# Strict lint & test suite (9/9 unit tests, 0 warnings)
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

# Run full E2E verification & Ghostty HUD receipt
cargo run --release --bin ayephone
```

Profile pinned in [`Cargo.toml`](../../Cargo.toml):
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

---

## 2. `MLX-QUANT` C++ & Metal 4 Ahead-of-Time (AOT) Kernel Lane

In [`8b-is/MLX-QUANT`](https://github.com/8b-is/MLX-QUANT):

```bash
cmake -B build -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DMLX_BUILD_METAL=ON \
  -DMLX_METAL_DEBUG=OFF \
  -DMLX_METAL_JIT=OFF \
  -DCMAKE_OSX_DEPLOYMENT_TARGET=14.0 \
  -DCMAKE_OSX_ARCHITECTURES=arm64
cmake --build build --config Release -j
```

### Metal Shader Compilation (`xcrun metal`)
```bash
xcrun -sdk macosx metal -c -O3 -ffast-math \
  -frecord-sources -gline-tables-only -mo \
  mlx/backend/metal/kernels/*.metal
```
- `-ffast-math` is confined to integer-packed quantized kernels (`{-1, 0, +1}` exact ternary decode + fused multiply-add pipeline).
- Hardware family tuning is delegated to Metal Pipeline State Objects (`MTLCreateSystemDefaultDevice` across M1 Pro and M3 Pro/Max) rather than brittle host C++ `-mcpu` flags.

---

## 3. Constellation Verification Matrix

| Lane | Toolchain | Target | Receipt |
| :--- | :--- | :--- | :--- |
| **`omnifold-core` & `ayephone` CLI** | `cargo 1.99.0` (ed2021) | `aarch64-apple-darwin` | **● 9/9 tests pass, clippy `-D warnings` clean** |
| **`MLX-QUANT` Metal 4 Kernels** | CMake + Ninja + `xcrun metal` | `aarch64-apple-darwin` | **● 12.80× storage, 1.2–5.5× fused kernel speedup** |
| **`MLX-QUANT` Golden-Logits Gate** | `python3 -O -m unittest` | Host / offline | **● 11/11 regressions pass at `ac6cf680`** |
| **`ayeOS` MEMNET Daemon** | `cargo` | `aarch64-apple-darwin` | **● 11/11 tests pass (`LINOSV` seed)** |
| **`8b-is-engine` (`admissibility` + `spherepop`)** | `cargo` | `aarch64-apple-darwin` | **● 11 unit + 1 doctest pass (`PR #4` merged)** |
| **`zeno-core` (`KintsugiPack` `ZKP1`)** | `uv run python -m unittest` | Host | **● 34/34 tests pass (4.24× chain compression)** |

*matrix closed · every flag reproducible · 0 + 1 · 8b.is / vaked.dev*
