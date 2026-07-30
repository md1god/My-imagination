use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KnowledgeEntry {
    pub generation: u64,
    pub topic: String,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Memory {
    pub generation: u64,
    pub age: u64,
    pub curiosity: f64,
    pub created_files: Vec<String>,
    pub core_rules: Vec<String>,
    #[serde(default)]
    pub explored_topics: Vec<String>,
    #[serde(default)]
    pub knowledge: Vec<KnowledgeEntry>,
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

    /// يمنع تراكم ملفات لا نهائي: يحتفظ بآخر `keep` ملف بس، والباقي يتحذف من القرص وذاكرة الملف.
    pub fn prune_old_files(&mut self, keep: usize) {
        while self.created_files.len() > keep {
            let oldest = self.created_files.remove(0);
            let _ = fs::remove_file(&oldest);
        }
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
                "قاعدة البحث الحر: ابحث عن معلومات حقيقية من الإنترنت وتعلم منها فعلياً.".to_string(),
            ],
            explored_topics: vec![],
            knowledge: vec![],
        }
    }
}
