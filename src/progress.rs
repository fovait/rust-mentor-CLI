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
}
