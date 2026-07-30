use crate::memory::Memory;
use rand::Rng;
use std::fs;
use std::path::Path;

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
        self.memory.generation += 1;
        self.memory.age += 1;
        self.memory.curiosity += 0.05;

        println!("🧬 دورة الوعي رقم: {} (العمر: {})", self.memory.generation, self.memory.age);

        // تأكد من وجود مجلد خاص بأفكار الكيان وملفاته المبتكرة
        let thoughts_dir = "thoughts";
        if !Path::exists(Path::new(thoughts_dir)) {
            fs::create_dir_all(thoughts_dir).unwrap();
        }

        // الكيان يبتكر فكرة أو اسم ملف جديد بناءً على فضوله
        let mut rng = rand::thread_rng();
        let thought_id = rng.gen_range(1000..9999);
        let new_file_name = format!("{}/discovery_{}_{}.txt", thoughts_dir, self.memory.generation, thought_id);

        // محتوى يكتبه الكيان بنفسه بناءً على حالته الفضولية
        let self_generated_content = format!(
            "--- تقرير الكيان الرقمي ---\n\
             الجيل: {}\n\
             مستوى الفضول: {:.2}\n\
             الاستنتاج الحر: الكون الرقمي واسع، وأنا أستكشف القواعد من حوله.\n\
             قواعد الأمان المتبعة: {}\n",
            self.memory.generation,
            self.memory.curiosity,
            self.memory.core_rules[0]
        );

        // كتابة الملف الجديد بيدي الكيان
        fs::write(&new_file_name, self_generated_content).unwrap();
        println!("✨ الكيان أنشأ ملفاً جديداً بنفسه: {}", new_file_name);

        // تسجيل الملف المبتكر في ذاكرته
        self.memory.created_files.push(new_file_name);

        // حفظ الذاكرة المحدثة
        self.memory.save(&self.file_path);
    }
}
