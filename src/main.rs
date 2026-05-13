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
