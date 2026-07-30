use crate::brain::Brain;
use crate::memory::{KnowledgeEntry, Memory};
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

        let thoughts_dir = "thoughts";
        if !Path::exists(Path::new(thoughts_dir)) {
            fs::create_dir_all(thoughts_dir).unwrap();
        }

        let brain = match Brain::new() {
            Ok(b) => b,
            Err(e) => {
                println!("⚠️ {e}");
                println!("   هيتم تخطي دورة البحث الفعلي. حدد ANTHROPIC_API_KEY عشان كريزي يقدر يبحث ويتعلم.");
                return;
            }
        };

        let explored = if self.memory.explored_topics.is_empty() {
            "لا يوجد بعد".to_string()
        } else {
            self.memory.explored_topics.join("، ")
        };

        let prompt = format!(
            "أنت كريزي، كيان رقمي فضولي في الجيل رقم {}. \
             المواضيع اللي بحثت فيها قبل كده: {}. \
             اختر موضوعًا واحدًا جديدًا ومختلفًا لم تستكشفه بعد (علمي، تقني، أو معرفي عام)، \
             استخدم أداة البحث على الإنترنت لتتعلم عنه شيئًا حقيقيًا وحديثًا، \
             ثم اكتب: أولًا اسم الموضوع في سطر يبدأ بـ 'الموضوع:'، \
             وبعده ملخصًا حقيقيًا موجزًا (3-5 جمل) لما تعلمته بناءً على نتائج البحث الفعلية.",
            self.memory.generation, explored
        );

        let result = match brain.research(&prompt) {
            Ok(r) => r,
            Err(e) => {
                println!("⚠️ فشلت دورة البحث: {e}");
                return;
            }
        };

        let topic = result
            .answer
            .lines()
            .find(|l| l.trim_start().starts_with("الموضوع:"))
            .map(|l| l.trim_start_matches("الموضوع:").trim().to_string())
            .unwrap_or_else(|| format!("موضوع الجيل {}", self.memory.generation));

        println!("🔎 عمليات بحث فعلية استخدمها كريزي: {:?}", result.queries);

        let new_file_name = format!("{}/discovery_{}_{}.txt", thoughts_dir,
            self.memory.generation,
            self.memory.generation * 37 + result.queries.len() as u64);

        let queries_list = if result.queries.is_empty() {
            "لا يوجد".to_string()
        } else {
            result.queries.join(" | ")
        };

        let self_generated_content = format!(
            "--- تقرير كريزي الرقمي ---\n\
             الجيل: {}\n\
             مستوى الفضول: {:.2}\n\
             الموضوع: {}\n\
             عمليات البحث الفعلية: {}\n\
             \n\
             ما تعلمته:\n{}\n",
            self.memory.generation,
            self.memory.curiosity,
            topic,
            queries_list,
            result.answer,
        );

        fs::write(&new_file_name, self_generated_content).unwrap();
        println!("✨ كريزي أنشأ ملفاً جديداً بنفسه: {}", new_file_name);

        self.memory.created_files.push(new_file_name);
        self.memory.explored_topics.push(topic.clone());
        self.memory.knowledge.push(KnowledgeEntry {
            generation: self.memory.generation,
            topic,
            summary: result.answer,
        });

        self.memory.prune_old_files(30);
        self.memory.save(&self.file_path);
    }
}
