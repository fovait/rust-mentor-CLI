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

/// List all lesson IDs from a directory, sorted numerically.
/// Extracts the 3-digit prefix from filenames like "001-hello-world.md".
pub fn list_ids(lesson_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ids: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(lesson_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".md") {
            let prefix: String = name.chars().take(3).collect();
            // Only collect if the first 3 chars are all ASCII digits
            if prefix.len() == 3 && prefix.chars().all(|c| c.is_ascii_digit()) {
                ids.push(prefix);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

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
}
