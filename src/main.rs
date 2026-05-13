use std::env;
use std::process;

use rust_mentor::lesson;
use rust_mentor::progress::{self, Progress};
use std::io::{self, Write};

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
