# Enums & Pattern Matching

Concept (2 min)

Enums let you define a type that is **one of several variants**. Unlike enums in
other languages, Rust enum variants can hold data.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),  // holds 4 bytes
    V6(String),           // holds a string
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

`match` is the tool for working with enums. The compiler forces you to handle
**every variant** — if you miss one, it won't compile.

```rust
match home {
    IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
    IpAddr::V6(addr)       => println!("IPv6: {}", addr),
}
```

Two enums you'll use everywhere in Rust:
- `Option<T>` — `Some(T)` or `None` (no null in Rust!)
- `Result<T, E>` — `Ok(T)` or `Err(E)` (no exceptions!)

---

Example (5 min)

```rust
#[derive(Debug)]
enum Message {
    Text(String),
    Image { url: String, width: u32 },  // named fields
    Heart,                               // no data
    Quit,
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Text(content) => format!("Text: {}", content),
            Message::Image { url, width: _ } => format!("Image from {}", url),
            Message::Heart => String::from("❤️"),
            Message::Quit => String::from("Bye!"),
        }
    }
}

fn main() {
    let msgs = vec![
        Message::Text(String::from("Hello!")),
        Message::Image { url: String::from("photo.png"), width: 800 },
        Message::Heart,
    ];

    for msg in &msgs {
        println!("{}", msg.describe());
    }

    // Option<T> — Rust's replacement for null
    let maybe_msg = msgs.get(5);  // Option<&Message>
    match maybe_msg {
        Some(msg) => println!("Found: {}", msg.describe()),
        None      => println!("No message at index 5"),
    }

    // if let — shorthand for single-variant match
    if let Some(first) = msgs.first() {
        println!("First: {}", first.describe());
    }
}
```

`if let` is syntactic sugar for `match` with one branch. Both do the same thing.

---

Why It Matters (2 min)

Enums + `match` is the heart of Rust's error handling and control flow:

- **No null** — `Option<T>` makes absence explicit. You can't accidentally use
  a value that might not exist without first checking.
- **No exceptions** — `Result<T, E>` makes failure explicit. Callers must decide
  how to handle errors; they can't ignore them by accident.
- **Exhaustive checking** — `match` must cover all variants. Add a new variant
  and the compiler points to every `match` that needs updating.

This combination means entire categories of bugs (null pointer, unhandled errors)
are caught at compile time. No runtime surprises.

In `rust-mentor`:
- `Option<String>` — `next_lesson_id()` returns `Some(id)` or `None`
- `Result<Progress, Box<dyn Error>>` — load/save return success or failure
- `Box<dyn Error>` itself is a sneak peek at traits (next lesson!)

---

Try It (1 min)

1. Define a `TrafficLight` enum with variants `Red`, `Yellow`, `Green`
2. Implement `next(&self) -> TrafficLight` that returns the next state
3. Print the sequence: Green → Yellow → Red → Green
4. Bonus: Define a `Shape` enum with `Circle(f64)`, `Rectangle(f64, f64)`,
   and `Triangle(f64, f64)`. Implement `area(&self) -> f64`.
