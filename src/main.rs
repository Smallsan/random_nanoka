use std::time::{SystemTime, UNIX_EPOCH};

struct NanokaRandom {
    seed: u64,
    nanoka_characters: Vec<&'static str>,
}

impl NanokaRandom {
    fn new() -> NanokaRandom {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_secs();

        let nanoka_characters = vec![
            "Hachikuji Mayoi",
            "Araragi Koyomi",
            "Senjougahara Hitagi",
            "Kanbaru Suruga",
            "Hanekawa Tsubasa",
            "Oshino Shinobu",
        ];

        NanokaRandom {
            seed,
            nanoka_characters,
        }
    }

    fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(self.nanoka_characters.len() as u64);
        self.seed
    }

    fn next_in_range(&mut self, min: u64, max: u64) -> u64 {
        min + self.next() % (max - min + 1)
    }
}

fn main() {
    let mut rng = NanokaRandom::new();
    
    for _ in 0..10 {
        let random_value = rng.next_in_range(1, 100);
        println!("Random Nanoka Number: {}", random_value);
    }
}