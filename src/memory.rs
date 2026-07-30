use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KnowledgeItem {
    pub question: String,
    pub experiment: String,
    pub result: String,
    pub timestamp: DateTime<Utc>,
    pub lesson: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Memory {
    pub age: u64,
    pub curiosity: f64,
    pub knowledge: Vec<KnowledgeItem>,
    pub rules: Vec<String>,
    pub generation: u64,
    pub last_evolution: DateTime<Utc>,
}

impl Memory {
    pub fn load_or_create(file_path: &str) -> Self {
        if Path::new(file_path).exists() {
            let data = fs::read_to_string(file_path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_else(|_| Memory::default())
        } else {
            Memory::default()
        }
    }

    pub fn save(&self, file_path: &str) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(file_path, json).unwrap();
    }

    fn default() -> Self {
        Memory {
            age: 0,
            curiosity: 1.0,
            knowledge: vec![],
            rules: vec![],
            generation: 1,
            last_evolution: Utc::now(),
        }
    }
}
