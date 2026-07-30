use serde_json::{json, Value};
use std::env;

pub struct ResearchResult {
    pub answer: String,
    pub queries: Vec<String>,
}

pub struct Brain {
    api_key: String,
    model: String,
}

impl Brain {
    /// يبني الاتصال بالمخ الفعلي. يفشل برسالة واضحة لو المفتاح مش موجود.
    pub fn new() -> Result<Self, String> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "❌ متغير البيئة ANTHROPIC_API_KEY غير موجود. لازم تضيفه قبل التشغيل.".to_string())?;

        // موديل سريع ورخيص مناسب لتشغيل تلقائي كل ساعة. ينفع تغيّره بمتغير CLAUDE_MODEL.
        let model = env::var("CLAUDE_MODEL").unwrap_or_else(|_| "claude-haiku-4-5-20251001".to_string());

        Ok(Brain { api_key, model })
    }

    /// يطلب من الموديل يبحث فعليًا على الإنترنت عن موضوع، ويرجع إجابة حقيقية + الاستعلامات اللي استخدمها.
    pub fn research(&self, prompt: &str) -> Result<ResearchResult, String> {
        let body = json!({
            "model": self.model,
            "max_tokens": 1024,
            "tools": [
                { "type": "web_search_20250305", "name": "web_search" }
            ],
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let response = ureq::post("https://api.anthropic.com/v1/messages")
            .set("x-api-key", &self.api_key)
            .set("anthropic-version", "2023-06-01")
            .set("content-type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .send_json(body);

        let parsed: Value = match response {
            Ok(resp) => resp
                .into_json()
                .map_err(|e| format!("فشل تحليل الرد: {e}"))?,
            Err(ureq::Error::Status(code, resp)) => {
                let text = resp.into_string().unwrap_or_default();
                return Err(format!("خطأ من API ({code}): {text}"));
            }
            Err(e) => return Err(format!("فشل الاتصال بـ API: {e}")),
        };

        let mut answer = String::new();
        let mut queries: Vec<String> = Vec::new();

        if let Some(blocks) = parsed["content"].as_array() {
            for block in blocks {
                match block["type"].as_str() {
                    Some("text") => {
                        if let Some(t) = block["text"].as_str() {
                            answer.push_str(t);
                            answer.push('\n');
                        }
                    }
                    // بلوكات استخدام أداة البحث (web_search) بتسجل الاستعلام اللي بحث بيه فعليًا
                    Some(kind) if kind.contains("tool_use") => {
                        if let Some(q) = block["input"]["query"].as_str() {
                            queries.push(q.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        if answer.trim().is_empty() {
            return Err(format!("رد غير متوقع من API (مفيش نص): {parsed}"));
        }

        Ok(ResearchResult {
            answer: answer.trim().to_string(),
            queries,
        })
    }
}
