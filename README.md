# rust-mentor

Daily Rust programming lessons in your terminal.

## Quick Start

```bash
cargo run -- today       # Show today's lesson
cargo run -- progress    # View your streak and stats
cargo run -- --help      # All commands
```

## Commands

| Command | Description |
|---|---|
| `rust-mentor today` | Show the next uncompleted lesson, mark it done |
| `rust-mentor progress` | Show streak, completion rate, progress bar |
| `rust-mentor --help` | Usage info |
| `rust-mentor --version` | Version number |

## How It Works

Lessons live as markdown files in `lessons/`. Each lesson has four sections:

- **Concept** (2 min) — What you're learning
- **Example** (5 min) — Code you can run
- **Why It Matters** (2 min) — Context and motivation
- **Try It** (1 min) — Hands-on exercise

Progress is tracked in `~/.rust-mentor.json` with streak and completion data.

## Project Structure

```
src/
  main.rs       — CLI dispatch and terminal rendering
  lesson.rs     — Markdown parsing and lesson loading
  progress.rs   — JSON progress I/O and streak logic
  lib.rs        — Module wiring
lessons/        — Lesson content (.md files)
```

## Requirements

- Rust toolchain (install via [rustup](https://rustup.rs))
