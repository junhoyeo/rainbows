use clap::{AppSettings, Clap};
use rand::prelude::*;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(long, default_value = "0")]
    number_of_rainbows: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let number_of_rainbows =
        if opts.number_of_rainbows == 0 {
            rand::thread_rng().gen_range(1..1000)
        } else {
            opts.number_of_rainbows as usize
        };

    let verb = if number_of_rainbows > 1 { "are" } else { "is" };
    let plurality = if number_of_rainbows > 1 { "s" } else { "" };
    println!(
        "Here {} {} rainbow{} for you!",
        verb, number_of_rainbows, plurality
    );
    println!("{}", "🌈".repeat(number_of_rainbows));
}
