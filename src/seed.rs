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
        Seed {
            memory,
            file_path: file_path.to_string(),
        }
    }

    pub fn cycle(&mut self) {
        self.memory.age += 1;
        self.memory.generation += 1;
        self.memory.curiosity += 0.001 * self.memory.age as f64;

        let question = self.generate_question();
        println!("🤔 [الجيل {}]: {}", self.memory.generation, question);

        let (experiment, result) = self.perform_experiment();
        println!("🌐 تجربة رقمية/إنترنت: {} => {}", experiment, result);

        let lesson = self.learn(&question, &experiment, &result);
        if let Some(ref l) = lesson {
            println!("📚 تعلم حقيقي: {}", l);
        }

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
        let base_questions = vec![
            "ماذا يقول العالم الخارجي الآن؟",
            "هل هناك حكمة جديدة يمكنني تعلمها؟",
            "كيف تتفاعل البيانات مع بيئتي الرقمية؟",
            "ماذا لو بحثت عن مصدر جديد للمعرفة؟",
        ];
        let mut rng = rand::thread_rng();
        base_questions[rng.gen_range(0..base_questions.len())].to_string()
    }

    fn perform_experiment(&self) -> (String, String) {
        let mut rng = rand::thread_rng();
        
        // بنسبة 50% تحاول النواة الاتصال بالإنترنت وجلب حكمة حقيقية!
        if rng.gen::<f64>() < 0.5 {
            let url = "https://api.adviceslip.com/advice";
            if let Ok(resp) = blocking::get(url) {
                if let Ok(data) = resp.json::<AdviceSlip>() {
                    return ("فحص الإنترنت (عالم خارجي)".to_string(), data.slip.advice);
                }
            }
        }

        // إن لم يتوفر الإنترنت أو سقط الاختيار على التجربة الداخلية
        let number: u32 = rng.gen_range(1..100);
        let result = number * 2;
        ("تجارب رياضية داخلية".to_string(), result.to_string())
    }

    fn learn(&mut self, question: &str, experiment: &str, result: &str) -> Option<String> {
        if experiment.contains("إنترنت") {
            let lesson = format!("استقبلت معرفة خارجية جديدة وقمت بتخزينها: '{}'", result);
            return Some(lesson);
        }
        None
    }
}
