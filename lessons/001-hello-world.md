# Hello, Rust!

Concept (2 min)

Rust is a systems programming language focused on safety, speed, and concurrency.
It achieves memory safety without a garbage collector, making it ideal for
performance-critical applications — from operating system kernels to web servers.

The Rust toolchain includes:
- `rustc` — the compiler
- `cargo` — the package manager and build system
- `rustup` — the toolchain installer and version manager

A Rust program starts in `main.rs` with a `fn main()` function, just like C.

---

Example (5 min)

```rust
fn main() {
    println!("Hello, world!");
    println!("I'm learning Rust!");
}
```

Save this as `main.rs` and run it:

```
$ rustc main.rs && ./main
Hello, world!
I'm learning Rust!
```

Or with Cargo (the standard way):

```
$ cargo new hello-rust
$ cd hello-rust
$ cargo run
Hello, world!
```

`println!` is a macro (note the `!`). Macros in Rust expand into code at compile
time. We'll explore macros much later — for now, just know that `println!` is
how you print to the terminal.

---

Why It Matters (2 min)

This is where every Rust programmer begins. The `println!` macro is your first
debugging tool — when something doesn't work, you'll reach for it to inspect
values and trace program flow.

More importantly, `cargo` is not just a build tool. It manages dependencies,
runs tests, builds documentation, and publishes packages. Nearly every Rust
project you encounter uses `cargo`. The `cargo new` command you just ran
created a standardized project structure that the entire Rust ecosystem shares.

---

Try It (1 min)

1. Create a new Cargo project called `hello-rust`
2. Modify `main.rs` to print your name and a fun fact about yourself
3. Run it with `cargo run`
4. Now try `cargo build --release` and compare the binary size of
   `target/debug/hello-rust` vs `target/release/hello-rust`
