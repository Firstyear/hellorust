use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..100);
    println!("Hello, Rust! The number of the day is: {}", n);
}
