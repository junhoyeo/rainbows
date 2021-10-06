use rand::prelude::*;

fn main() {
    let random_int = rand::thread_rng().gen_range(1..1000);
    let number_of_rainbows = random_int;

    let verb = if number_of_rainbows > 1 { "are" } else { "is" };
    let plurality = if number_of_rainbows > 1 { "s" } else { "" };
    println!(
        "Here {} {} rainbow{} for you!",
        verb, number_of_rainbows, plurality
    );
    println!("{}", "🌈".repeat(number_of_rainbows));
}
