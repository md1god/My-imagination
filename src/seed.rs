use crate::memory::{Memory, KnowledgeItem};
use rand::Rng;
use chrono::Utc;
use reqwest::blocking;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AdviceSlip {
    slip: Slip,
}

#[derive(Deserialize, Debug)]
struct Slip {
    advice: String,
}

pub struct Seed {
    pub memory: Memory,
    file_path: String,
}

impl Seed {
    pub fn new(file_path: &str) -> Self {
        let memory = Memory::load_or_create(file_path);
        Seed { memory, file_path: file_path.to_string() }
    }

    pub fn cycle(&mut self) {
        self.memory.age += 1;
        self.memory.generation += 1;
        self.memory.curiosity += 0.001 * self.memory.age as f64;

        let question = self.generate_question();
        println!("🤔 [الجيل {}]: {}", self.memory.generation, question);

        let (experiment, result) = self.perform_experiment();
        println!("🌐 تجربة: {} => {}", experiment, result);

        let lesson = Some(format!("نتيجة التجربة: {}", result));

        let item = KnowledgeItem {
            question,
            experiment,
            result,
            timestamp: Utc::now(),
            lesson,
        };
        self.memory.knowledge.push(item);
        self.memory.last_evolution = Utc::now();
        self.memory.save(&self.file_path);
    }

    fn generate_question(&self) -> String {
        let questions = vec![
            "ماذا يقول العالم الخارجي الآن؟",
            "هل هناك حكمة جديدة يمكنني تعلمها؟",
            "كيف تتفاعل البيانات مع بيئتي الرقمية؟",
        ];
        let mut rng = rand::thread_rng();
        questions[rng.gen_range(0..questions.len())].to_string()
    }

    fn perform_experiment(&self) -> (String, String) {
        let url = "https://api.adviceslip.com/advice";
        if let Ok(resp) = blocking::get(url) {
            if let Ok(data) = resp.json::<AdviceSlip>() {
                return ("فحص الإنترنت".to_string(), data.slip.advice);
            }
        }
        ("تجربة محلية".to_string(), "فشل الاتصال بالخارج".to_string())
    }
}
