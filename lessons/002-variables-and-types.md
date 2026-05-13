# Variables, Types, and Mutability

Concept (2 min)

Rust is statically typed — every value has a type known at compile time. But you
rarely write types explicitly because Rust's type inference figures them out.

Key types you'll use every day:

| Type | Example | Use |
|---|---|---|
| `i32` | `42`, `-10` | Signed integer (default) |
| `u32` | `0`, `100` | Unsigned integer (no negatives) |
| `f64` | `3.14`, `2.0` | Floating-point (default) |
| `bool` | `true`, `false` | Boolean |
| `String` | `"hello".to_string()` | Owned text on the heap |
| `&str` | `"hello"` | Borrowed text slice |

Variables are **immutable by default**. Add `mut` to make them mutable:

```rust
let x = 5;      // immutable — can't change x
let mut y = 5;  // mutable — can change y
```

This is a deliberate design choice. Rust nudges you toward fewer mutations,
which means fewer bugs.

---

Example (5 min)

```rust
fn main() {
    // Type inference — Rust knows these are i32, f64, bool
    let answer = 42;
    let pi = 3.14159;
    let is_rust_fun = true;

    // Explicit types (optional, but sometimes helpful for clarity)
    let small: u8 = 255;  // u8 fits 0..255
    let name: String = String::from("Rustacean");

    println!("Answer: {}", answer);
    println!("Pi: {:.2}", pi);
    println!("Fun? {}", is_rust_fun);
    println!("Name: {}", name);

    // Immutable by default — this would be a compile error:
    // answer = 43;  // error[E0384]: cannot assign twice to immutable variable

    // Mutable variables CAN change:
    let mut counter = 0;
    println!("Counter starts at: {}", counter);
    counter += 1;
    println!("Counter is now: {}", counter);

    // Shadowing — create a new variable with the same name
    let spaces = "   ";            // &str
    let spaces = spaces.len();     // usize — shadows the old `spaces`
    println!("Spaces length: {}", spaces);
}
```

Run this and try uncommenting the `answer = 43` line to see the compiler error.
The compiler message will tell you exactly what's wrong and suggest adding `mut`.

---

Why It Matters (2 min)

Immutability-by-default is one of Rust's most important design decisions. It:

1. **Prevents accidental mutations** — if you don't say `mut`, you can't
   accidentally change a value. Many bugs come from unexpected mutations.
2. **Enables the borrow checker** — Rust's ownership system (next lesson!) relies
   on knowing what can and can't change. `mut` is a signal to both the compiler
   and other programmers.
3. **Makes code easier to reason about** — when you see `let x = 5`, you know
   `x` will be `5` throughout its lifetime. No need to scan for mutations.

Shadowing (`let spaces = spaces.len()`) is different from mutation. It creates
a *new* variable that happens to share a name. This is useful for transforming
values without needing to invent names like `spaces_str`, `spaces_len`, etc.

In our `rust-mentor` tool, we use these concepts everywhere:
- `let mut progress` — we modify progress when a lesson is completed
- `pub struct Progress` — custom types with named fields
- `Vec<String>` — a vector (growable array) of owned strings

---

Try It (1 min)

1. Create a new Cargo project: `cargo new variables-playground`
2. Write a program that:
   - Declares an immutable variable and tries to modify it (watch it fail)
   - Fixes it with `mut`
   - Uses shadowing to transform a string into its length
   - Prints everything
3. Run it: `cargo run`
4. Bonus: Try the `dbg!` macro for quick debugging:
   `dbg!(counter);` — it prints the file, line number, and value
