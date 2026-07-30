use crate::memory::Memory;
use rand::Rng;

pub struct Seed {
    pub memory: Memory,
    file_path: String,
}

impl Seed {
    pub fn new(file_path: &str) -> Self {
        let mut memory = Memory::load_or_create(file_path);
        if memory.population.is_empty() {
            memory.population = Self::init_population(20);
        }
        Seed { memory, file_path: file_path.to_string() }
    }

    fn init_population(size: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let mut pop = Vec::new();
        let ops = ['+', '-', '*'];
        for _ in 0..size {
            let op1 = ops[rng.gen_range(0..ops.len())];
            let op2 = ops[rng.gen_range(0..ops.len())];
            let val1: i32 = rng.gen_range(-10..10);
            let val2: i32 = rng.gen_range(-10..10);
            let code = format!("x {} {} {} {} {}", op1, val1, op2, val2, rng.gen_range(1..5));
            pop.push(code);
        }
        pop
    }

    fn fitness(code: &str) -> f64 {
        let test_vals = [-5, -2, 0, 3, 7];
        let mut total_error = 0.0;

        for &x in &test_vals {
            let target = (x * 2 + 5) as f64;
            let out = Self::eval_code(code, x);
            if out.is_infinite() || out.is_nan() {
                return f64::MAX;
            }
            total_error += (out - target).abs();
        }
        total_error / test_vals.len() as f64
    }

    fn eval_code(code: &str, x: i32) -> f64 {
        let parts: Vec<&str> = code.split_whitespace().collect();
        if parts.len() < 6 { return f64::MAX; }
        
        let op1 = parts[1];
        let val1: i32 = parts[2].parse().unwrap_or(1);
        let op2 = parts[3];
        let val2: i32 = parts[4].parse().unwrap_or(1);
        let multiplier: i32 = parts[5].parse().unwrap_or(1);

        let mut res = x;
        match op1 {
            "+" => res += val1,
            "-" => res -= val1,
            "*" => res *= val1,
            _ => {}
        }
        match op2 {
            "+" => res += val2,
            "-" => res -= val2,
            "*" => res *= val2,
            _ => {}
        }
        (res * multiplier) as f64
    }

    pub fn cycle(&mut self) {
        self.memory.generation += 1;
        println!("🧬 الجيل التطوري: {}", self.memory.generation);

        let mut scored: Vec<(String, f64)> = self.memory.population.iter()
            .map(|code| (code.clone(), Self::fitness(code)))
            .collect();

        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let (best_code, best_err) = scored[0].clone();

        if best_err < self.memory.best_score {
            self.memory.best_score = best_err;
            self.memory.best_expression = best_code.clone();
            println!("🌟 تحسن جيني جديد! أفضل خطأ: {:.4}", best_err);
            println!("💻 الكود الفائز: {}", best_code);
        } else {
            println!("📈 الأداء مستقر - أفضل خطأ حالياً: {:.4}", best_err);
        }

        let mut rng = rand::thread_rng();
        let mut new_pop = vec![scored[0].0.clone(), scored[1].0.clone()];

        while new_pop.len() < self.memory.population.len() {
            let p1 = &scored[rng.gen_range(0..scored.len() / 2)].0;
            let p2 = &scored[rng.gen_range(0..scored.len() / 2)].0;

            let mut child = if rng.gen::<bool>() { p1.clone() } else { p2.clone() };

            if rng.gen::<f64>() < 0.4 {
                let ops = ['+', '-', '*'];
                let op = ops[rng.gen_range(0..ops.len())];
                let val: i32 = rng.gen_range(-10..10);
                child = format!("x {} {} {} {} {}", op, val, op, rng.gen_range(-5..5), rng.gen_range(1..5));
            }

            new_pop.push(child);
        }

        self.memory.population = new_pop;
        self.memory.save(&self.file_path);
    }
}
