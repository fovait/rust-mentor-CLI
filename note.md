# Rust: Debug vs Release Builds

**Date:** 2026-05-13
**Tags:** #rust #cargo #toolchain #debug-vs-release

## Summary

| | `cargo run` (debug) | `cargo build --release` (release) |
|---|---|---|
| Binary location | `target/debug/` | `target/release/` |
| Compile speed | Fast | Slow (2-10x longer) |
| Runtime speed | Slow | Fast (10-100x+ faster) |
| Debug info | Yes | Stripped |
| Optimization | None | Aggressive (inlining, loop unrolling, dead code elimination) |
| Binary size | Larger | Smaller |
| Use case | Development, iteration, testing | Shipping, benchmarking, production |

## Key Insight

`cargo run` (or `cargo build`) compiles without optimizations for fast iteration. `cargo build --release` invokes LLVM's aggressive optimizer — for CPU-bound code the difference can be dramatic (e.g., the compiler may optimize an entire loop away). For I/O-bound programs (like `rust-mentor`), the difference is negligible.

## Rule of Thumb
- `cargo run` while building
- `cargo build --release` for the real thing
