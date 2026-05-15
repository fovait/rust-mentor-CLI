# References & Borrowing

Concept (2 min)

Ownership is strict — moving values into functions means you lose them. References
solve this: they let you *use* a value without *taking* it.

```rust
let s = String::from("hello");
let len = calculate_length(&s);  // &s = reference to s, we don't move it
println!("{} has length {}", s, len);  // s still valid!
```

The `&` creates a **reference** — a pointer that lets you look at the value
without owning it. This action is called **borrowing**. When the reference goes
out of scope, the original value stays put.

Two kinds of references:
- `&T` — **shared reference** (read-only, many at once)
- `&mut T` — **exclusive reference** (read-write, only one at a time)

The compiler enforces one rule at compile time:
> **Shared XOR mutable** — you can have many `&T` OR one `&mut T`, never both.

This is the single rule that eliminates data races at compile time.

---

Example (5 min)

```rust
fn main() {
    let mut s = String::from("hello");

    // Immutable borrow — read, don't take
    let len = calc_len(&s);
    println!("'{}' has {} chars", s, len);  // s still here

    // Mutable borrow — modify without taking
    append_world(&mut s);
    println!("After append: {}", s);

    // Multiple shared borrows — fine
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);  // both valid

    // Mutable borrow after shared — fine, shared references are gone
    let r3 = &mut s;
    r3.push_str("!!!");
    println!("{}", r3);
}

fn calc_len(s: &String) -> usize {  // borrows, doesn't own
    s.len()
}  // s (the reference) goes out of scope, original String untouched

fn append_world(s: &mut String) {
    s.push_str(", world!");
}
```

Try to break the rules — the compiler will stop you:
```rust
let mut s = String::from("test");
let r1 = &s;       // shared borrow
let r2 = &mut s;   // ERROR: can't mut borrow while shared borrow exists
println!("{}", r1);
```

---

Why It Matters (2 min)

This is Rust's killer feature. The compiler prevents:
- **Dangling references** — can't return a reference to a dropped value
- **Data races** — can't have shared + mutable access at the same time
- **Iterator invalidation** — can't modify a collection while iterating

Languages without borrow checking (C, C++) let these compile — they become
runtime crashes or security vulnerabilities. Languages with GC (Java, Go)
prevent some but pay runtime cost.

Rust catches all of it at compile time. Zero runtime overhead.

In `rust-mentor`, references are everywhere:
- `load(path: &Path)` — borrows the path, doesn't need to own it
- `mark_completed(&mut self, ...)` — borrows self mutably to update progress
- `lessons.iter()` — returns references to each lesson

---

Try It (1 min)

1. Write a function `longest_word(s: &str) -> usize` that returns the length
   of the longest word in a string slice
2. Write a function `append_exclamation(s: &mut String)` that adds `"!"`
3. In `main`, call both and print results — show that borrowing works
4. Bonus: Try creating a `&mut` reference while a `&` reference is still alive.
   Read the compiler error — it explains the problem and suggests a fix.
