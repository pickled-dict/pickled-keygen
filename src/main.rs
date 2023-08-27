use clap::Parser;
use rand::seq::SliceRandom;
use available_chars::AvailableChars;
use chars_options::CharsOptions;

mod available_chars;
mod chars_options;

const ABOUT_STRING: &str = "Generate a random string for things like \
    secret keys and passwords.\nGives a 16 character key with \
    upper, lower, symbols, and numbers on by default";

#[derive(Parser, Debug)]
#[command(name = "Pickled Keygen")]
#[command(author = "pickled-dict")]
#[command(version = "0.1.0")]
#[command(
    about = ABOUT_STRING,
    long_about=None
)]
struct Args {
    /// Adds uppercase letters to character set
    #[arg(short, long)]
    upper: bool,

    /// Adds lowercase letters to character set
    #[arg(short, long)]
    lower: bool,

    /// Adds symbols to character set
    #[arg(short, long)]
    symbols: bool,

    /// Adds numbers to character set
    #[arg(short, long)]
    numbers: bool,

    /// Length of the generated string
    #[arg(short, long, default_value_t = 16)]
    count: usize,
}

fn generate_string(available: AvailableChars, size: usize) -> String {
    (0..size).map(|_| available.chars.choose(&mut rand::thread_rng()).unwrap())
        .collect()
}

fn main() {
    let args = Args::parse();

    let options = CharsOptions {
        upper: args.upper,
        lower: args.lower,
        symbols: args.symbols,
        numbers: args.numbers
    };

    if options.numbers == false && options.lower == false && options.symbols == false && options.upper == false {
        let deluxe = AvailableChars::builder().uppercase().lowercase().symbols().numbers().build();
        print!("{}", generate_string(deluxe, args.count));
    } else {
        let options_available = AvailableChars::builder().from_options(options).build();
        print!("{}", generate_string(options_available, args.count));
    }
}
