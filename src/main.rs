use available_chars::AvailableChars;
use chars_options::CharsOptions;
use rand::seq::SliceRandom;

mod available_chars;
mod chars_options;

fn generate_string(available: AvailableChars, size: usize) -> String {
    (0..size).map(|_| available.chars.choose(&mut rand::thread_rng()).unwrap())
        .collect()
}

fn main() {
    let options = CharsOptions {upper: true, lower: false, numbers: false, symbols: false};
    let available = AvailableChars::builder().from_options(options).build();

    let generated = generate_string(available, 50);

    println!("{}", generated);
}
