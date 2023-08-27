use available_chars::AvailableChars;
use chars_options::CharsOptions;
use clap::Parser;
use utils::generate_string;

mod available_chars;
mod chars_options;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT_STRING: &str = "Generate a random string for things like \
    secret keys and passwords.\nGives a 16 character key with \
    upper, lower, symbols, and numbers on by default";

#[derive(Parser, Debug)]
#[command(name = "Pickled Keygen")]
#[command(author = "pickled-dict")]
#[command(version = VERSION)]
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

fn main() {
    // initialize clap arg parser
    let args = Args::parse();

    // get string character settings from args
    let options = CharsOptions {
        upper: args.upper,
        lower: args.lower,
        symbols: args.symbols,
        numbers: args.numbers,
    };

    match options.is_empty() {
        true => print!(
            "{}",
            generate_string(AvailableChars::builder().default_options().build(), args.count)
        ),
        false => print!(
            "{}",
            generate_string(
                AvailableChars::builder().from_options(options).build(),
                args.count
            )
        ),
    }
}
