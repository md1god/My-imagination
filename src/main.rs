mod memory;
mod seed;

use seed::Seed;
use std::env;

fn main() {
    println!("🌱 Genesis Seed بدأ التطور الذاتي الجيني");
    let file_path = "memory.json";
    let mut seed = Seed::new(file_path);

    let cycles = env::var("CYCLES")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3);

    for i in 0..cycles {
        println!("\n--- دورة الحوسبة السحابية {} ---", i + 1);
        seed.cycle();
    }
    println!("✅ اكتملت الدورات وحُفظت الذاكرة الجينية.");
}
