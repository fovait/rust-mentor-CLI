use std::error::Error;

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
