use available_chars::AvailableChars;
use chars_options::CharsOptions;
mod available_chars;
mod chars_options;

fn main() {
    let options = CharsOptions {upper: true, lower: false, numbers: true, symbols: false};
    let available_with_options = AvailableChars::builder().from_options(options).build();

    println!("{:?}", available_with_options.chars);
}
