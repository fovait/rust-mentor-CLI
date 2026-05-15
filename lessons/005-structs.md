# Structs: Defining Your Own Types

Concept (2 min)

Structs group related data into a single type. Think of them as custom
containers — you define the shape, Rust enforces the structure.

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
    is_available: bool,
}
```

Three ways to create a struct:

```rust
// 1. Named fields (most common)
let book = Book {
    title: String::from("The Rust Book"),
    author: String::from("Steve Klabnik"),
    pages: 560,
    is_available: true,
};

// 2. Field init shorthand — when variable name matches field name
let title = String::from("The Rust Book");
let author = String::from("Steve Klabnik");
let book = Book { title, author, pages: 560, is_available: true };

// 3. Struct update — copy fields from another instance
let book2 = Book {
    title: String::from("The Rust Book 2"),
    ..book  // copies remaining fields
};
// Note: book.title still valid (String not copied), but book.author was moved!
```

Structs can also have methods via `impl` blocks:

```rust
impl Book {
    fn description(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}
println!("{}", book.description());
```

---

Example (5 min)

```rust
#[derive(Debug)]
struct Student {
    name: String,
    score: u32,
    grade: char,
}

impl Student {
    // Constructor — a convention, not built-in
    fn new(name: String, score: u32) -> Self {
        let grade = match score {
            90..=100 => 'A',
            80..=89  => 'B',
            70..=79  => 'C',
            60..=69  => 'D',
            _        => 'F',
        };
        Self { name, score, grade }
    }

    fn passed(&self) -> bool {
        self.grade != 'F'
    }

    // &mut self — modify the instance
    fn curve(&mut self, points: u32) {
        self.score += points;
        self.grade = match self.score {
            90..=100 => 'A',
            80..=89  => 'B',
            70..=79  => 'C',
            60..=69  => 'D',
            _        => 'F',
        };
    }
}

fn main() {
    let mut alice = Student::new(String::from("Alice"), 85);
    println!("{:?}", alice);
    println!("Passed: {}", alice.passed());

    alice.curve(5);
    println!("After curve: {:?}", alice);
}
```

`#[derive(Debug)]` auto-generates the `Debug` trait so `{:?}` can print the
struct for debugging. Without it, you can't print custom types.

---

Why It Matters (2 min)

Structs are the backbone of Rust programs. Every non-trivial program defines its
own types. Key design decisions:

- **Structs own their fields by default** — `Book` owns its `title: String`. This
  means when the struct is dropped, all its fields are dropped together.
- **Methods take `&self` by default** — you share-borrow the struct unless you
  need mutation (`&mut self`) or ownership (`self`). Same borrowing rules apply.
- **No inheritance** — Rust uses composition and traits instead. Structs don't
  inherit; they implement traits (next lesson!).

In `rust-mentor`, everything is a struct:
- `Lesson { title, concept, example, why, try_it }` — lesson data
- `Progress { completed, streak, started_at }` — user state
- `Streak { current, last_completed_date, longest }` — nested struct

---

Try It (1 min)

1. Define a `Task` struct with `title` (String), `done` (bool), `priority` (u8)
2. Implement `new(title)`, `complete(&mut self)`, and `label(&self)`
3. Create 3 tasks, complete one, print all labels
4. Bonus: add a `Project` struct that contains a `Vec<Task>` and a name.
   Write a method `remaining(&self)` that counts incomplete tasks.
