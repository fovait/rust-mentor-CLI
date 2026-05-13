# Rust Mentor CLI — Design Spec

## Overview

`rust-mentor` is a CLI tool that delivers daily Rust programming lessons via local markdown files. It is both a learning tool AND the first Rust project built by its user — each Rust concept taught maps directly to a piece of the tool itself.

The V1 core loop: run `rust-mentor today`, read a lesson, mark it complete, see your streak.

## Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Lesson storage | Local markdown files (`lessons/*.md`) | Transparent, editable, version-controllable. Parsing markdown is a good early Rust exercise. |
| Progress tracking | JSON file (`~/.rust-mentor.json`) | Simple, human-readable, easy to debug. `serde_json` is idiomatic Rust. |
| Lesson sequence | Completion-gated | Always show the next uncompleted lesson. No time pressure. Streak counts consecutive days with at least one completion. |
| CLI framework | Manual arg parsing (V1) | Only two commands. Teaches what frameworks like `clap` do before adding them later. |
| Crate structure | Single binary crate | Simplest starting point. Natural refactor to library+CLI when codebase grows. |

## Lesson File Format

Markdown files in `lessons/` named like `001-hello-world.md`. The numeric prefix determines ordering; the slug is human-readable.

Structure — four sections delimited by `---`:

```markdown
# Title Here

Concept (2 min)

...

---

Example (5 min)

```rust
...
```

---

Why It Matters (2 min)

...

---

Try It (1 min)

...

```

The title is extracted from the first `#` heading. Sections are split on the `---` delimiter. No YAML frontmatter, no regex — plain string splitting with `str::split()`.

## Progress File Schema

Location: `~/.rust-mentor.json`

```json
{
  "completed": ["001", "002", "003"],
  "streak": {
    "current": 3,
    "last_completed_date": "2026-05-14",
    "longest": 5
  },
  "started_at": "2026-05-12T08:00:00Z",
  "version": 1
}
```

### Streak Logic

- Complete a lesson today → increment `current`, update `last_completed_date` to today
- If `current` > `longest`, update `longest`
- `last_completed_date` was yesterday and no completion today → streak hibernates (current stays)
- `last_completed_date` was 2+ days ago → `current` resets to 0 (or 1 if completing today)

## CLI Interface

### Commands

| Command | Behavior |
|---|---|
| `rust-mentor today` | Show the next uncompleted lesson. Offer to mark complete. |
| `rust-mentor progress` | Show streak, completion stats, progress bar. |
| `rust-mentor --help` / `-h` | Usage summary. |
| `rust-mentor --version` / `-v` | Version number. |

### `today` Output

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Day 4: Ownership Basics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Concept (2 min)
  ────────────────
  ...

  Example (5 min)
  ────────────────
  ...

  Why It Matters (2 min)
  ────────────────
  ...

  Try It (1 min)
  ────────────────
  ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Mark as completed? (y/n)
```

### `progress` Output

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Streak:        5 days 🔥
  Longest streak: 5 days
  Completed:      5 / 90 lessons (6%)
  Started:        May 12, 2026
  Progress:       [████░░░░░░░░░░░░░░] 6%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Edge Cases

- **All lessons completed:** "You've completed all available lessons! Check back soon for more."
- **No lessons directory:** "No lessons found. Expected a `lessons/` directory in the current working directory."
- **Already completed a lesson today:** Show the next lesson anyway with streak info: "You're on a {N}-day streak!"
- **First run (no progress file):** Create default progress, show day 1 lesson.
- **Corrupt progress file:** Show error with path, suggest deleting and restarting.

## Code Architecture

Single binary crate with four modules:

```
src/
  main.rs       — CLI dispatch, terminal rendering, "Mark complete?" prompt
  lesson.rs     — Lesson struct, markdown parser, filesystem loading
  progress.rs   — Progress struct, JSON read/write, streak logic
  lib.rs        — Re-exports, ties lesson + progress together
```

### `lesson.rs`

```rust
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub concept: String,
    pub example: String,
    pub why: String,
    pub try_it: String,
}

pub fn list_ids(lesson_dir: &Path) -> Result<Vec<String>>;
pub fn load(lesson_dir: &Path, id: &str) -> Result<Lesson>;
fn parse_markdown(raw: &str) -> Result<Lesson>;
```

### `progress.rs`

```rust
pub struct Progress {
    pub completed: Vec<String>,
    pub streak: Streak,
    pub started_at: DateTime<Utc>,
    pub version: u32,
}

pub struct Streak {
    pub current: u32,
    pub last_completed_date: Option<NaiveDate>,
    pub longest: u32,
}

pub fn load(path: &Path) -> Result<Progress>;
pub fn save(path: &Path, progress: &Progress) -> Result<()>;
pub fn mark_completed(progress: &mut Progress, lesson_id: &str);
pub fn next_lesson_id(progress: &Progress, all_ids: &[String]) -> Option<String>;
```

### `main.rs`

Manual arg dispatch — `std::env::args()` and `match`. Two command branches (`today`, `progress`) plus help and version. Terminal output uses plain `println!` with box-drawing characters.

### Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
```

Three standard dependencies:
- `serde` + `serde_json` — progress file serialization
- `chrono` — date comparison and formatting

### Directory Layout at Rest

```
~/.rust-mentor.json              ← progress file (created on first run)
./lessons/001-hello-world.md     ← lessons in CWD (looked up relative to where you run the command)
./lessons/002-ownership.md
...
```

## Testing

- **Unit tests:** `parse_markdown` (various inputs, edge cases), streak calculation (today, yesterday, gap, edge of day)
- **Integration tests:** `next_lesson_id` given various completion states, progress load/save round-trip

## Error Handling

All fallible operations return `Result<T, Box<dyn Error>>` for V1. The `main` function prints errors to stderr and exits non-zero. Custom error types can be added later as a "Rust error handling" lesson.

## Future (Post-V1)

| Feature | Notes |
|---|---|
| `rust-mentor review N` | Show the last N lessons as a recap |
| `rust-mentor search <term>` | Search lesson titles and content |
| Daily email delivery | SMTP or email API integration |
| Spaced repetition | Re-show lessons at intervals, difficulty self-rating |
| `clap` migration | Replace manual arg parsing with `clap` derive API |
| Library crate extraction | Split into `rust-mentor-lib` + `rust-mentor` binary |
