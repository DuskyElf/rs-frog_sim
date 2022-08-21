use rand::{self, Rng};

const ITTERATIONS: u64 = 1_000_000;

fn main() {
    let mut sum = 0;
    for _ in 0..ITTERATIONS {
        let result = frog_simulation() as u64;
        sum += result;
    }

    println!("Average: {}", sum as f64 / ITTERATIONS as f64);
}

fn frog_simulation() -> u8 {
    let mut jumps = 0u8;
    let mut space = 0u8;

    while space != 10 {
        space = rand::thread_rng().gen_range((space + 1)..=10);
        jumps += 1;
    }

    jumps
}
