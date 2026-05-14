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

---

# Rust: Shadowing (`let x = x.len()`)

**Date:** 2026-05-14
**Tags:** #rust #variable-shadowing #immutability #let

## Summary

Shadowing is declaring a **new variable** with the same name as an existing one, hiding the old one. It is NOT mutation — it's a new binding that can have a different type, and doesn't need `mut`.

```rust
let spaces = "   ";       // &str
let spaces = spaces.len(); // usize — new variable, shadowed
```

## Why It Was Designed This Way

1. **Transform values without inventing names** — avoid `input_trimmed`, `input_parsed` suffixes. Keep the same name through a pipeline of transformations.

2. **Enforce immutability through transformations** — after `let data = validate(data)`, the raw unvalidated data is inaccessible. Compiler prevents accidental use of stale intermediate values.

3. **Avoid unnecessary `mut`** — use a block to create a mutable intermediate, then shadow the immutable result back to the original name.

## Key Insight

Shadowing is a **compile-time concept** with zero runtime cost. The old variable simply goes out of scope. The compiler may reuse the stack slot if types are compatible.

```rust
// Not mutation (would fail — can't change type)
let mut x = "hello";
x = x.len(); // ERROR

// Shadowing (OK — new variable)
let x = "hello";
let x = x.len(); // OK — x is now usize
```

---

# Rust: `dbg!` Macro

**Date:** 2026-05-14
**Tags:** #rust #debugging #macro #dbg

## Summary

`dbg!` prints **file, line number, and value** to stderr, then returns ownership of the value — so it can be used inline without breaking code flow.

```rust
let x = 5;
let y = dbg!(x);  // [src/main.rs:3] x = 5
```

## `dbg!` vs `println!`

| | `dbg!` | `println!` |
|---|---|---|
| Output | stderr | stdout |
| Location | Auto (file + line) | Manual |
| Returns value | Yes (ownership) | No (returns `()`) |
| Format | `{:?}` (Debug trait) | Custom format string |

## Key Insight

Because `dbg!` returns the value, you can wrap any expression inline:

```rust
// Wrap an expression mid-chain
let result = dbg!(expensive_calculation());

// In iterator chains
let numbers: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|n| dbg!(n * 2))  // see each step
    .collect();

// Borrow to avoid move
let counter = 42;
dbg!(&counter);  // borrows, doesn't consume
```

---

# Rust: `.iter()` vs `.into_iter()`

**Date:** 2026-05-14
**Tags:** #rust #iterators #ownership #borrowing

## Summary

| | `.iter()` | `.into_iter()` |
|---|---|---|
| Yields | `&T` (references) | `T` (owned values) |
| Borrows or owns? | Borrows | Takes ownership |
| Original collection | Still usable | Consumed (gone) |
| Get owned value | Must `.clone()` (allocates) | Free — transfers ownership |
| Use when | Reading, need collection later | Extracting values, last use |

## Key Insight

`.into_iter()` lets you extract owned values from a collection without cloning — the Vec is destroyed but its elements live on in their new home. `.iter()` can only give you references.

```rust
let words = vec![String::from("hello"), String::from("world")];

// .iter() — need .cloned() to get owned String (allocates new memory)
let cloned: Vec<String> = words.iter().cloned().collect();

// .into_iter() — takes Strings directly (zero allocation)
let owned: Vec<String> = words.into_iter().collect();
```

**Rule of thumb:** `.iter()` to read, `.into_iter()` to extract. If you find yourself writing `.iter().cloned()` everywhere, you probably want `.into_iter()`.
