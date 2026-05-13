# Rust Mentor CLI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a `rust-mentor` CLI that delivers daily Rust programming lessons from local markdown files, tracks completion and streaks in a JSON progress file.

**Architecture:** Single binary crate with four modules — `main.rs` (CLI dispatch, terminal rendering), `lesson.rs` (markdown parsing, filesystem loading), `progress.rs` (JSON I/O, streak math), `lib.rs` (module wiring). Manual arg parsing for V1. TDD throughout.

**Tech Stack:** Rust, serde/serde_json for serialization, chrono for dates

---

### Task 1: Scaffold the Rust project

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`

- [ ] **Step 1: Initialize the Cargo project**

```bash
cargo init --name rust-mentor
```

- [ ] **Step 2: Write `Cargo.toml` with dependencies**

```toml
[package]
name = "rust-mentor"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
```

- [ ] **Step 3: Write `src/lib.rs` with module declarations**

```rust
pub mod lesson;
pub mod progress;
```

- [ ] **Step 4: Create placeholder module files so the project compiles**

Write `src/lesson.rs`:
```rust
// Lesson loading and markdown parsing
```

Write `src/progress.rs`:
```rust
// Progress tracking and streak logic
```

- [ ] **Step 5: Verify the scaffold compiles**

```bash
cargo build
```
Expected: Compiles successfully (may have unused import warnings, that's fine).

- [ ] **Step 6: Verify tests run (none yet)**

```bash
cargo test
```
Expected: "running 0 tests"

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml src/
git commit -m "feat: scaffold rust-mentor project with dependencies"
```

---

### Task 2: Lesson struct and markdown parsing

**Files:**
- Modify: `src/lesson.rs`

- [ ] **Step 1: Write the `Lesson` struct and a failing test for `parse_markdown`**

Replace `src/lesson.rs` with:

```rust
use std::error::Error;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub concept: String,
    pub example: String,
    pub why: String,
    pub try_it: String,
}

/// Parse a lesson markdown string into a Lesson struct.
/// Expects 4 sections delimited by `\n---\n`.
/// The first section must start with a `# Title` heading.
fn parse_markdown(raw: &str, id: &str) -> Result<Lesson, Box<dyn Error>> {
    let sections: Vec<&str> = raw.split("\n---\n").collect();
    if sections.len() != 4 {
        return Err(format!(
            "Expected 4 sections delimited by ---, found {}",
            sections.len()
        )
        .into());
    }

    let title = sections[0]
        .lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line[2..].trim().to_string())
        .ok_or("Missing title line starting with '# '")?;

    Ok(Lesson {
        id: id.to_string(),
        title,
        concept: sections[0].trim().to_string(),
        example: sections[1].trim().to_string(),
        why: sections[2].trim().to_string(),
        try_it: sections[3].trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_valid() {
        let raw = "\
# Hello World

Concept (2 min)

Rust is a systems programming language that runs blazingly fast.

---

Example (5 min)

```rust
fn main() {
    println!(\"Hello, world!\");
}
```

---

Why It Matters (2 min)

Every Rust programmer starts here. The `println!` macro is your first
tool for seeing what your program does.

---

Try It (1 min)

Change the message inside println! to print your own name.
Run `rustc` or `cargo run` to see the output.
";

        let lesson = parse_markdown(raw, "001").unwrap();

        assert_eq!(lesson.id, "001");
        assert_eq!(lesson.title, "Hello World");
        assert!(lesson.concept.contains("blazingly fast"));
        assert!(lesson.example.contains("println!"));
        assert!(lesson.why.contains("starts here"));
        assert!(lesson.try_it.contains("your own name"));
    }

    #[test]
    fn test_parse_markdown_missing_title() {
        let raw = "\
No title here

---

Example

---

Why

---

Try It
";
        let result = parse_markdown(raw, "001");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing title"));
    }

    #[test]
    fn test_parse_markdown_wrong_section_count() {
        let raw = "\
# Title

Just one section, no delimiters.
";
        let result = parse_markdown(raw, "001");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("4 sections"));
    }
}
```

- [ ] **Step 2: Run the tests — should pass**

```bash
cargo test
```
Expected: 3 tests pass (test_parse_markdown_valid, test_parse_markdown_missing_title, test_parse_markdown_wrong_section_count)

- [ ] **Step 3: Commit**

```bash
git add src/lesson.rs
git commit -m "feat: add Lesson struct and markdown parsing"
```

---

### Task 3: Lesson filesystem operations — list_ids and load

**Files:**
- Modify: `src/lesson.rs`

- [ ] **Step 1: Add tests for `list_ids` and `load`**

Append to the `tests` module in `src/lesson.rs`:

```rust
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_list_ids() {
        let dir = std::env::temp_dir().join("rust-mentor-test-list");
        fs::create_dir_all(&dir).unwrap();

        let mut f1 = fs::File::create(dir.join("001-hello.md")).unwrap();
        writeln!(f1, "# Hello").unwrap();

        let mut f2 = fs::File::create(dir.join("002-ownership.md")).unwrap();
        writeln!(f2, "# Ownership").unwrap();

        // This file should be ignored (not .md)
        let mut f3 = fs::File::create(dir.join("README.txt")).unwrap();
        writeln!(f3, "not a lesson").unwrap();

        let ids = list_ids(&dir).unwrap();
        assert_eq!(ids, vec!["001", "002"]);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_load() {
        let dir = std::env::temp_dir().join("rust-mentor-test-load");
        fs::create_dir_all(&dir).unwrap();

        let content = "\
# Ownership Basics

Concept (2 min)

In Rust, every value has exactly one owner at a time.

---

Example (5 min)

```rust
let s1 = String::from(\"hello\");
let s2 = s1;
```

---

Why It Matters (2 min)

Ownership is the foundation of Rust's memory safety model.

---

Try It (1 min)

Create a String, move it, and try to use the original.
";
        let mut f = fs::File::create(dir.join("003-ownership.md")).unwrap();
        writeln!(f, "{}", content).unwrap();

        let lesson = load(&dir, "003").unwrap();
        assert_eq!(lesson.id, "003");
        assert_eq!(lesson.title, "Ownership Basics");
        assert!(lesson.concept.contains("exactly one owner"));

        // Non-existent lesson
        let result = load(&dir, "999");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        fs::remove_dir_all(&dir).unwrap();
    }
```

- [ ] **Step 2: Run tests — should fail (list_ids, load not defined)**

```bash
cargo test
```
Expected: compile errors — `list_ids` and `load` not found.

- [ ] **Step 3: Implement `list_ids` and `load`**

Add above the `#[cfg(test)]` block in `src/lesson.rs`:

```rust
/// List all lesson IDs from a directory, sorted numerically.
/// Extracts the 3-digit prefix from filenames like "001-hello-world.md".
pub fn list_ids(lesson_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ids: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(lesson_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".md") && name.len() >= 3 {
            let prefix = &name[..3];
            // Only collect if the first 3 chars are all ASCII digits
            if prefix.chars().all(|c| c.is_ascii_digit()) {
                ids.push(prefix.to_string());
            }
        }
    }
    ids.sort();
    Ok(ids)
}

/// Load a lesson by ID from the lessons directory.
/// Finds the file starting with "{id}-" and parses it.
pub fn load(lesson_dir: &Path, id: &str) -> Result<Lesson, Box<dyn Error>> {
    let mut file_path: Option<std::path::PathBuf> = None;
    for entry in std::fs::read_dir(lesson_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(&format!("{}-", id)) && name.ends_with(".md") {
            file_path = Some(entry.path());
            break;
        }
    }

    let path = file_path.ok_or_else(|| format!("Lesson {} not found in {:?}", id, lesson_dir))?;
    let raw = std::fs::read_to_string(&path)?;
    parse_markdown(&raw, id)
}
```

- [ ] **Step 4: Add `use` imports at top of `src/lesson.rs`**

The top of the file should read:

```rust
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
```

- [ ] **Step 5: Run tests — should pass**

```bash
cargo test
```
Expected: All 5 tests pass.

- [ ] **Step 6: Clean up unused import warnings**

If the compiler warns about unused imports (e.g., `Write`), prefix the test-only uses with `#[cfg(test)]` or add `#![allow(unused_imports)]` at the crate level. For now, verify:

```bash
cargo build
```
Expected: No errors. Warnings are acceptable for test imports.

- [ ] **Step 7: Commit**

```bash
git add src/lesson.rs
git commit -m "feat: add lesson filesystem operations (list_ids, load)"
```

---

### Task 4: Progress struct and JSON I/O

**Files:**
- Modify: `src/progress.rs`

- [ ] **Step 1: Write Progress structs and a serialization round-trip test**

Replace `src/progress.rs` with:

```rust
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Progress {
    pub completed: Vec<String>,
    pub streak: Streak,
    pub started_at: DateTime<Utc>,
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Streak {
    pub current: u32,
    pub last_completed_date: Option<NaiveDate>,
    pub longest: u32,
}

impl Progress {
    /// Create a fresh Progress for a first-time user.
    pub fn new() -> Self {
        Progress {
            completed: vec![],
            streak: Streak {
                current: 0,
                last_completed_date: None,
                longest: 0,
            },
            started_at: Utc::now(),
            version: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_serialize_deserialize() {
        let progress = Progress {
            completed: vec!["001".to_string(), "002".to_string()],
            streak: Streak {
                current: 2,
                last_completed_date: NaiveDate::from_ymd_opt(2026, 5, 14),
                longest: 5,
            },
            started_at: DateTime::from_timestamp(0, 0).unwrap(),
            version: 1,
        };

        let json = serde_json::to_string_pretty(&progress).unwrap();
        let deserialized: Progress = serde_json::from_str(&json).unwrap();
        assert_eq!(progress, deserialized);
        // Verify the JSON contains expected fields
        assert!(json.contains("\"completed\""));
        assert!(json.contains("\"streak\""));
        assert!(json.contains("\"current\": 2"));
        assert!(json.contains("\"version\": 1"));
    }

    #[test]
    fn test_new_progress() {
        let p = Progress::new();
        assert!(p.completed.is_empty());
        assert_eq!(p.streak.current, 0);
        assert_eq!(p.streak.longest, 0);
        assert_eq!(p.streak.last_completed_date, None);
        assert_eq!(p.version, 1);
    }
}
```

- [ ] **Step 2: Run tests — the serialization test should pass**

```bash
cargo test
```
Expected: 7 tests pass (2 new + 5 from lesson module).

- [ ] **Step 3: Add `save` and `load` functions with a round-trip test**

Append to the `tests` module in `src/progress.rs`:

```rust
    #[test]
    fn test_save_and_load_round_trip() {
        let dir = std::env::temp_dir().join("rust-mentor-test-progress");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test-progress.json");

        let original = Progress {
            completed: vec!["001".to_string(), "002".to_string(), "003".to_string()],
            streak: Streak {
                current: 3,
                last_completed_date: NaiveDate::from_ymd_opt(2026, 5, 14),
                longest: 7,
            },
            started_at: DateTime::from_timestamp(0, 0).unwrap(),
            version: 1,
        };

        save(&path, &original).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(original, loaded);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load(Path::new("/tmp/does-not-exist-12345.json"));
        assert!(result.is_err());
    }
```

- [ ] **Step 4: Run tests — should fail (save, load not defined)**

```bash
cargo test
```
Expected: compile errors for `save` and `load`.

- [ ] **Step 5: Implement `save` and `load`**

Add above the `#[cfg(test)]` block in `src/progress.rs`:

```rust
/// Save progress to a JSON file.
pub fn save(path: &Path, progress: &Progress) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(progress)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Load progress from a JSON file.
pub fn load(path: &Path) -> Result<Progress, Box<dyn Error>> {
    let raw = std::fs::read_to_string(path)?;
    let progress: Progress = serde_json::from_str(&raw)?;
    Ok(progress)
}
```

- [ ] **Step 6: Run tests — should pass**

```bash
cargo test
```
Expected: All 9 tests pass (4 new + 5 from lesson module).

- [ ] **Step 7: Commit**

```bash
git add src/progress.rs
git commit -m "feat: add Progress struct with JSON save/load"
```

---

### Task 5: Streak logic and completion tracking

**Files:**
- Modify: `src/progress.rs`

- [ ] **Step 1: Add tests for `mark_completed`**

Append to the `tests` module in `src/progress.rs`:

```rust
    #[test]
    fn test_mark_completed_first_time() {
        let mut p = Progress::new();
        mark_completed(&mut p, "001");

        assert_eq!(p.completed, vec!["001"]);
        assert_eq!(p.streak.current, 1);
        assert_eq!(p.streak.longest, 1);
        assert!(p.streak.last_completed_date.is_some());
        // last_completed_date should be today
        let today = chrono::Local::now().date_naive();
        assert_eq!(p.streak.last_completed_date.unwrap(), today);
    }

    #[test]
    fn test_mark_completed_same_day_no_double_count() {
        let today = chrono::Local::now().date_naive();
        let mut p = Progress {
            completed: vec!["001".to_string()],
            streak: Streak {
                current: 1,
                last_completed_date: Some(today),
                longest: 1,
            },
            started_at: Utc::now(),
            version: 1,
        };

        mark_completed(&mut p, "001");
        // Already completed, should not add again
        assert_eq!(p.completed, vec!["001"]);
        assert_eq!(p.streak.current, 1); // no change
    }

    #[test]
    fn test_mark_completed_consecutive_day() {
        let yesterday = chrono::Local::now().date_naive()
            .pred_opt().unwrap();
        let mut p = Progress {
            completed: vec!["001".to_string()],
            streak: Streak {
                current: 1,
                last_completed_date: Some(yesterday),
                longest: 1,
            },
            started_at: Utc::now(),
            version: 1,
        };

        mark_completed(&mut p, "002");
        assert_eq!(p.streak.current, 2);
        assert_eq!(p.streak.longest, 2); // longest updated
        let today = chrono::Local::now().date_naive();
        assert_eq!(p.streak.last_completed_date.unwrap(), today);
    }

    #[test]
    fn test_mark_completed_after_gap() {
        let three_days_ago = chrono::Local::now().date_naive()
            .pred_opt().unwrap()
            .pred_opt().unwrap()
            .pred_opt().unwrap();
        let mut p = Progress {
            completed: vec!["001".to_string()],
            streak: Streak {
                current: 1,
                last_completed_date: Some(three_days_ago),
                longest: 5, // previous best streak
            },
            started_at: Utc::now(),
            version: 1,
        };

        mark_completed(&mut p, "002");
        assert_eq!(p.streak.current, 1); // reset
        assert_eq!(p.streak.longest, 5); // longest preserved
    }

    #[test]
    fn test_mark_completed_beats_longest() {
        let yesterday = chrono::Local::now().date_naive()
            .pred_opt().unwrap();
        let mut p = Progress {
            completed: vec!["001".to_string(), "002".to_string()],
            streak: Streak {
                current: 2,
                last_completed_date: Some(yesterday),
                longest: 2,
            },
            started_at: Utc::now(),
            version: 1,
        };

        mark_completed(&mut p, "003");
        assert_eq!(p.streak.current, 3);
        assert_eq!(p.streak.longest, 3);
    }
```

- [ ] **Step 2: Run tests — should fail (mark_completed not defined)**

```bash
cargo test
```
Expected: compile error — `mark_completed` not found.

- [ ] **Step 3: Implement `mark_completed`**

Add above the `#[cfg(test)]` block in `src/progress.rs`:

```rust
/// Mark a lesson as completed and update streak.
/// Does nothing if the lesson was already completed.
pub fn mark_completed(progress: &mut Progress, id: &str) {
    // Don't double-count
    if progress.completed.contains(&id.to_string()) {
        return;
    }

    progress.completed.push(id.to_string());

    let today = chrono::Local::now().date_naive();

    match progress.streak.last_completed_date {
        None => {
            // First completion ever
            progress.streak.current = 1;
        }
        Some(last_date) => {
            let diff = (today - last_date).num_days();
            if diff == 0 {
                // Same-day completion, do nothing (already handled by the early return above)
                // This branch is unreachable for new IDs but kept for clarity
            } else if diff == 1 {
                // Consecutive day
                progress.streak.current += 1;
            } else {
                // Gap of 2+ days — streak resets
                progress.streak.current = 1;
            }
        }
    }

    progress.streak.last_completed_date = Some(today);

    if progress.streak.current > progress.streak.longest {
        progress.streak.longest = progress.streak.current;
    }
}
```

- [ ] **Step 4: Add tests for `next_lesson_id`**

Append to the `tests` module in `src/progress.rs`:

```rust
    #[test]
    fn test_next_lesson_id_returns_first_uncompleted() {
        let p = Progress {
            completed: vec!["001".to_string(), "002".to_string()],
            streak: Streak {
                current: 2,
                last_completed_date: None,
                longest: 2,
            },
            started_at: Utc::now(),
            version: 1,
        };
        let all_ids = vec![
            "001".to_string(),
            "002".to_string(),
            "003".to_string(),
            "004".to_string(),
        ];
        assert_eq!(next_lesson_id(&p, &all_ids), Some("003".to_string()));
    }

    #[test]
    fn test_next_lesson_id_all_complete() {
        let p = Progress {
            completed: vec!["001".to_string(), "002".to_string()],
            streak: Streak {
                current: 2,
                last_completed_date: None,
                longest: 2,
            },
            started_at: Utc::now(),
            version: 1,
        };
        let all_ids = vec!["001".to_string(), "002".to_string()];
        assert_eq!(next_lesson_id(&p, &all_ids), None);
    }

    #[test]
    fn test_next_lesson_id_none_completed() {
        let p = Progress::new();
        let all_ids = vec!["001".to_string(), "002".to_string()];
        assert_eq!(next_lesson_id(&p, &all_ids), Some("001".to_string()));
    }
```

- [ ] **Step 5: Run tests — should fail (next_lesson_id not defined)**

```bash
cargo test
```
Expected: compile error — `next_lesson_id` not found.

- [ ] **Step 6: Implement `next_lesson_id`**

Add below `mark_completed` in `src/progress.rs`:

```rust
/// Find the first lesson ID not in the completed list.
pub fn next_lesson_id(progress: &Progress, all_ids: &[String]) -> Option<String> {
    all_ids
        .iter()
        .find(|id| !progress.completed.contains(id))
        .cloned()
}
```

- [ ] **Step 7: Run tests — should all pass**

```bash
cargo test
```
Expected: All 16 tests pass (9 from progress + 5 from lesson + 2 from progress I/O). Verify with `cargo test`.

- [ ] **Step 8: Commit**

```bash
git add src/progress.rs
git commit -m "feat: add streak logic and next-lesson selection"
```

---

### Task 6: CLI skeleton — arg parsing, help, version

**Files:**
- Modify: `src/main.rs` (overwrite the default hello-world)

- [ ] **Step 1: Write the CLI skeleton**

Replace `src/main.rs` with:

```rust
use std::env;
use std::process;

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("today") => cmd_today(),
        Some("progress") => cmd_progress(),
        Some("--help") | Some("-h") => print_help(),
        Some("--version") | Some("-v") => print_version(),
        Some(unknown) => {
            eprintln!("Unknown command: {}\nTry --help", unknown);
            process::exit(1);
        }
        None => {
            eprintln!("No command provided.\nTry --help");
            process::exit(1);
        }
    }
}

fn cmd_today() {
    println!("TODO: show today's lesson");
}

fn cmd_progress() {
    println!("TODO: show progress");
}

fn print_help() {
    println!("rust-mentor — daily Rust lessons in your terminal\n");
    println!("USAGE:");
    println!("  rust-mentor <command>\n");
    println!("COMMANDS:");
    println!("  today       Show the next uncompleted lesson");
    println!("  progress    Show your streak and progress");
    println!("  --help, -h     Show this help message");
    println!("  --version, -v  Show version number");
}

fn print_version() {
    println!("rust-mentor v{}", VERSION);
}
```

- [ ] **Step 2: Verify it compiles and basic commands work**

```bash
cargo build
```

```bash
cargo run -- --help
```
Expected: Help text with today, progress, --help, --version.

```bash
cargo run -- --version
```
Expected: `rust-mentor v0.1.0`

```bash
cargo run -- today
```
Expected: `TODO: show today's lesson`

```bash
cargo run -- progress
```
Expected: `TODO: show progress`

```bash
cargo run
```
Expected: `No command provided.` with exit code 1.

```bash
cargo run -- bogus
```
Expected: `Unknown command: bogus` with exit code 1.

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: add CLI skeleton with arg parsing, help, and version"
```

---

### Task 7: Implement `cmd_today` — full lesson delivery flow

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add `use` imports at the top of `src/main.rs`**

Insert these `use` lines at the top of `src/main.rs`, after the existing `use std::env;` and `use std::process;`:

```rust
use rust_mentor::lesson;
use rust_mentor::progress::{self, Progress};
use std::io::{self, Write};
```

Then replace just the `cmd_today` function stub with:

```rust
fn cmd_today() {
    let progress_path = get_progress_path();
    let lesson_dir = std::path::Path::new("lessons");

    // Load or create progress
    let mut progress = match progress::load(&progress_path) {
        Ok(p) => p,
        Err(_) => {
            let p = Progress::new();
            if let Err(e) = progress::save(&progress_path, &p) {
                eprintln!("Error creating progress file: {}", e);
                process::exit(1);
            }
            p
        }
    };

    // List available lesson IDs
    let all_ids = match lesson::list_ids(&lesson_dir) {
        Ok(ids) => ids,
        Err(_) => {
            eprintln!(
                "No lessons found. Expected a `lessons/` directory in the current working directory."
            );
            process::exit(1);
        }
    };

    if all_ids.is_empty() {
        println!("No lessons found in the `lessons/` directory.");
        process::exit(1);
    }

    // Find the next uncompleted lesson
    let next_id = match progress::next_lesson_id(&progress, &all_ids) {
        Some(id) => id,
        None => {
            println!("You've completed all available lessons! Check back soon for more.");
            return;
        }
    };

    // Load and render the lesson
    let lesson = match lesson::load(&lesson_dir, &next_id) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error loading lesson {}: {}", next_id, e);
            process::exit(1);
        }
    };

    render_lesson(&lesson, &progress);

    // Ask to mark complete
    if prompt_complete() {
        progress::mark_completed(&mut progress, &next_id);
        if let Err(e) = progress::save(&progress_path, &progress) {
            eprintln!("Error saving progress: {}", e);
            process::exit(1);
        }
        println!("\nLesson marked complete! Streak: {} day(s)", progress.streak.current);
    }
}
```

- [ ] **Step 2: Add helper functions**

Add these functions to `src/main.rs` before the `fn main()` block (after the `use` statements):

```rust
/// Resolve the progress file path to ~/.rust-mentor.json
fn get_progress_path() -> std::path::PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home).join(".rust-mentor.json")
}

/// Render a lesson to the terminal
fn render_lesson(lesson: &lesson::Lesson, progress: &Progress) {
    let day = progress.completed.len() + 1;
    let bar = "━".repeat(36);

    println!("{}", bar);
    println!("  Day {}: {}", day, lesson.title);
    println!("{}", bar);

    if progress.streak.current > 0 {
        println!("\n  🔥 {}-day streak!\n", progress.streak.current);
    } else {
        println!();
    }

    let divider = "  ────────────────";

    println!("  Concept (2 min)");
    println!("{}", divider);
    println!("{}", indent_text(&lesson.concept, 2));

    println!("\n  Example (5 min)");
    println!("{}", divider);
    println!("{}", indent_text(&lesson.example, 2));

    println!("\n  Why It Matters (2 min)");
    println!("{}", divider);
    println!("{}", indent_text(&lesson.why, 2));

    println!("\n  Try It (1 min)");
    println!("{}", divider);
    println!("{}", indent_text(&lesson.try_it, 2));

    println!("\n{}", bar);
}

/// Indent every line of text by N spaces
fn indent_text(text: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Prompt the user: "Mark as completed? (y/n)" — return true if yes
fn prompt_complete() -> bool {
    print!("  Mark as completed? (y/n) ");
    io::stdout().flush().ok();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim().to_lowercase();
            trimmed == "y" || trimmed == "yes"
        }
        Err(_) => false,
    }
}
```

- [ ] **Step 3: Make the lesson and progress types accessible from main**

Update `src/lib.rs`:

```rust
pub mod lesson;
pub mod progress;
```

(The `pub mod` declarations are already there from Task 1. Verify that `lib.rs` reads exactly as above.)

- [ ] **Step 4: Verify it compiles**

```bash
cargo build
```
Expected: Compiles successfully.

- [ ] **Step 5: Create the lessons directory for smoke testing**

```bash
mkdir -p lessons
```

- [ ] **Step 6: Verify error messages for missing lessons**

```bash
cargo run -- today
```
Expected: "No lessons found..." error since the lessons directory is empty.

- [ ] **Step 7: Commit**

```bash
git add src/main.rs
git commit -m "feat: implement `today` command with lesson display and completion"
```

---

### Task 8: Implement `cmd_progress`

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace the `cmd_progress` stub**

Replace the `cmd_progress` function stub in `src/main.rs` with:

```rust
fn cmd_progress() {
    let progress_path = get_progress_path();

    let progress = match progress::load(&progress_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("No progress file found at {:?}", progress_path);
            eprintln!("Run `rust-mentor today` first to get started.");
            eprintln!("({})", e);
            process::exit(1);
        }
    };

    let lesson_dir = std::path::Path::new("lessons");
    let total_lessons = match lesson::list_ids(&lesson_dir) {
        Ok(ids) => ids.len(),
        Err(_) => 0,
    };

    let completed_count = progress.completed.len();
    let pct = if total_lessons > 0 {
        (completed_count as f64 / total_lessons as f64) * 100.0
    } else {
        0.0
    };

    let bar = "━".repeat(36);

    println!("{}", bar);
    println!("  Streak:        {} day(s) 🔥", progress.streak.current);
    println!("  Longest streak: {} day(s)", progress.streak.longest);
    println!(
        "  Completed:      {} / {} lessons ({:.0}%)",
        completed_count, total_lessons, pct
    );
    println!(
        "  Started:        {}",
        progress.started_at.format("%B %d, %Y")
    );
    println!("  Progress:       {}", render_progress_bar(pct));
    println!("{}", bar);
}

/// Render a simple ASCII progress bar like [████░░░░░░░░░░░░░░] 6%
fn render_progress_bar(pct: f64) -> String {
    let width = 20;
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let empty = width - filled;
    format!(
        "[{}{}] {:.0}%",
        "█".repeat(filled),
        "░".repeat(empty),
        pct
    )
}
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo build
```
Expected: Compiles successfully.

- [ ] **Step 3: Smoke test progress without a file**

```bash
rm -f ~/.rust-mentor.json
cargo run -- progress
```
Expected: Error message about no progress file, with suggestion to run `today` first.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat: implement `progress` command with stats and progress bar"
```

---

### Task 9: Write the first lesson

**Files:**
- Create: `lessons/001-hello-world.md`

- [ ] **Step 1: Create lesson 001**

```bash
mkdir -p lessons
```

Write `lessons/001-hello-world.md`:

```markdown
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
```

- [ ] **Step 2: Verify the full flow works end-to-end**

First, remove any existing progress:

```bash
rm -f ~/.rust-mentor.json
```

Run the today command:

```bash
cargo run -- today
```

Expected: Renders lesson 001 with all four sections. Prompts "Mark as completed? (y/n)".

Type `y` and press Enter. Expected: "Lesson marked complete! Streak: 1 day(s)".

Run progress:

```bash
cargo run -- progress
```

Expected: Shows streak of 1 day, 1 / 1 lesson completed (100%), progress bar full.

Run today again:

```bash
cargo run -- today
```

Expected: "You've completed all available lessons! Check back soon for more."

- [ ] **Step 3: Commit**

```bash
git add lessons/001-hello-world.md
git commit -m "feat: add first lesson — Hello, Rust!"
```

---

### Task 10: Edge cases and final verification

**Files:**
- Modify: `src/main.rs` (only if edge case handling needs fixes)

- [ ] **Step 1: Test corrupt progress file handling**

```bash
echo "not valid json" > ~/.rust-mentor.json
cargo run -- today
```

Expected: Should either show an error about the corrupt file, or create a fresh progress and continue. The current code creates a fresh `Progress` if loading fails, which is the graceful recovery path — but we should display a warning. Let's add that.

Update the progress-loading section in `cmd_today` (the `match progress::load(&progress_path)` block):

```rust
    let mut progress = match progress::load(&progress_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "Note: Could not read progress file ({}) — starting fresh.",
                e
            );
            let p = Progress::new();
            if let Err(e) = progress::save(&progress_path, &p) {
                eprintln!("Error creating progress file: {}", e);
                process::exit(1);
            }
            p
        }
    };
```

- [ ] **Step 2: Verify the corrupt-file recovery works**

```bash
echo "not valid json" > ~/.rust-mentor.json
cargo run -- today
```
Expected: Shows a note about corrupt progress, then shows lesson 001 (since the fresh progress has no completions).

- [ ] **Step 3: Run the full test suite one final time**

```bash
cargo test
```
Expected: All tests pass.

- [ ] **Step 4: Build the release binary**

```bash
cargo build --release
```
Expected: Compiles in release mode. Binary at `target/release/rust-mentor`.

- [ ] **Step 5: Final end-to-end verification with the release binary**

```bash
rm -f ~/.rust-mentor.json
./target/release/rust-mentor --version
./target/release/rust-mentor --help
./target/release/rust-mentor progress
./target/release/rust-mentor today   # Type 'y' at prompt
./target/release/rust-mentor progress
```

Expected: All commands produce the expected output as described in Task 9's verification.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs
git commit -m "fix: show warning when progress file is corrupt and recovering"
```

---

## Summary

After all 10 tasks, the project should look like:

```
Cargo.toml
src/
  lib.rs           — pub mod lesson; pub mod progress;
  lesson.rs        — Lesson struct, parse_markdown, list_ids, load
  progress.rs      — Progress/Streak structs, save/load, mark_completed, next_lesson_id
  main.rs          — CLI dispatch, rendering, cmd_today, cmd_progress
lessons/
  001-hello-world.md  — First lesson content
```

User-visible commands:
- `rust-mentor today` — show next uncompleted lesson, offer to mark complete
- `rust-mentor progress` — show streak, completion stats, progress bar
- `rust-mentor --help` / `-h` — usage
- `rust-mentor --version` / `-v` — version
