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
                // Same-day completion of a different lesson — no streak change.
                // Streak counts consecutive days, not lessons per day.
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

/// Find the first lesson ID not in the completed list.
pub fn next_lesson_id(progress: &Progress, all_ids: &[String]) -> Option<String> {
    all_ids
        .iter()
        .find(|id| !progress.completed.contains(id))
        .cloned()
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
}
