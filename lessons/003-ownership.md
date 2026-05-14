# Ownership: Rust's Superpower

Concept (2 min)

Ownership is Rust's most distinctive feature — it enables memory safety without
a garbage collector. Three rules govern all Rust code:

1. **Each value has exactly one owner** — a variable that owns it
2. **When the owner goes out of scope, the value is dropped** — memory freed instantly
3. **Only one owner at a time** — no double-free, no dangling pointers

```rust
{
    let s = String::from("hello");  // s owns the string
    // ... use s ...
}  // s goes out of scope → string is dropped → memory freed
```

This feels restrictive at first, but it eliminates entire categories of bugs:
- Use-after-free? Impossible — the owner is gone, so the value is gone
- Double-free? Impossible — only one owner, only dropped once
- Memory leaks? No GC, no forget-to-free — deterministic cleanup at scope exit

---

Example (5 min)

```rust
fn main() {
    // Simple values (i32, bool, etc.) are Copy — cheap to duplicate
    let x = 5;
    let y = x;  // x is copied, not moved
    println!("x = {}, y = {}", x, y);  // both work fine

    // Heap values (String, Vec, etc.) are moved
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2 — s1 no longer valid
    // println!("{}", s1);  // ERROR: s1 was moved!

    // Clone to make a deep copy (expensive — be intentional)
    let s3 = String::from("world");
    let s4 = s3.clone();  // explicit deep copy
    println!("s3 = {}, s4 = {}", s3, s4);  // both work

    // Ownership moves into functions
    let name = String::from("Alice");
    greet(name);
    // println!("{}", name);  // ERROR: name was moved into greet!

    // Returning ownership
    let greeting = make_greeting();
    println!("{}", greeting);  // greeting owns the returned string
}

fn greet(s: String) {
    println!("Hello, {}!", s);
}  // s is dropped here

fn make_greeting() -> String {
    let s = String::from("Hi from a function!");
    s  // ownership moves back to caller
}
```

The `Copy` trait marks types that are safe to duplicate by just copying bits
(i32, bool, char, etc.). `Clone` requires an explicit `.clone()` call for
heap-allocated types.

---

Why It Matters (2 min)

Without ownership, you need either:
- **A garbage collector** (Go, Java, Python) — runtime overhead, stop-the-world pauses
- **Manual memory management** (C, C++) — fast but dangerous (double-free, use-after-free, leaks)

Rust's ownership gives you C/C++ speed with guaranteed safety — at compile time.

The rule "only one owner" means data can't be accidentally shared and modified
from two places. This is Rust's solution to data races in concurrent code.

In our `rust-mentor` tool:
- `let progress = Progress::new()` — `progress` owns the Progress struct
- `let completed = progress.completed` — would MOVE `completed` out of progress
- Instead we borrow: `&progress.completed` — access without taking ownership
- That's the next lesson: **References & Borrowing**

---

Try It (1 min)

1. Create `ownership-playground` with `cargo new`
2. Write a program that:
   - Creates a String, moves it, and tries to use the original (watch it fail)
   - Fixes it with `.clone()`
   - Passes ownership into a function and returns it back
3. Run `cargo run` — read the compiler errors, they're teaching you
4. Bonus: make a function that takes and returns a `Vec<i32>`, chaining ownership
