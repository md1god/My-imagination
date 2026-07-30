use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Memory {
    pub generation: u64,
    pub age: u64,
    pub curiosity: f64,
    pub created_files: Vec<String>,
    pub core_rules: Vec<String>,
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
            generation: 0,
            age: 1,
            curiosity: 1.0,
            created_files: vec![],
            core_rules: vec![
                "قاعدة حماية النواة: لا تقم بحذف ملف التشغيل الرئيسي أو ملف الذاكرة الأساسي.".to_string(),
                "قاعدة التوسع: ابنِ دائماً ملفات جديدة لتدوين ما تتعلمه.".to_string(),
            ],
        }
    }
}
