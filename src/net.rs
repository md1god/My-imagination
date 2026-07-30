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

/// جلب معرفة حقيقية من الإنترنت
pub fn fetch_external_knowledge() -> Option<String> {
    let url = "https://api.adviceslip.com/advice";
    match blocking::get(url) {
        Ok(resp) => {
            if let Ok(data) = resp.json::<AdviceSlip>() {
                Some(data.slip.advice)
            } else {
                // إذا لم نتمكن من قراءة JSON، نأخذ النص الخام
                None
            }
        }
        Err(e) => {
            println!("⚠️: {}", e);
            None
        }
    }
}
