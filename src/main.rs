use clap::{App, Arg};
use rand::prelude::*;

const MAX_NUMBER_OF_RAINBOWS: u16 = 1000;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("number-of-rainbows")
            .short('n')
            .long("number-of-rainbows")
            .value_name("NUMBER")
            .about("Manually sets the number of rainbows")
            .default_value("0"))
        .get_matches();

    let given_number_of_rainbows: u16 = std::cmp::min(
        matches
            .value_of_t("number-of-rainbows")
            .unwrap_or(MAX_NUMBER_OF_RAINBOWS),
        MAX_NUMBER_OF_RAINBOWS,
    );
    let number_of_rainbows =
        if given_number_of_rainbows == 0 {
            rand::thread_rng().gen_range(1..MAX_NUMBER_OF_RAINBOWS)
        } else {
            given_number_of_rainbows
        } as usize;

    let verb = if number_of_rainbows > 1 { "are" } else { "is" };
    let plurality = if number_of_rainbows > 1 { "s" } else { "" };
    println!(
        "Here {} {} rainbow{} for you!",
        verb, number_of_rainbows, plurality
    );
    println!("{}", "🌈".repeat(number_of_rainbows));
}
