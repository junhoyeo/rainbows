use rand::prelude::*;

fn main() {
    let random_int = rand::thread_rng().gen_range(1..1000);
    println!("{}", "🌈".repeat(random_int));
}
