use available_chars::AvailableChars;
mod available_chars;

fn main() {
    let nls = AvailableChars::builder()
        .numbers()
        .lowercase()
        .symbols()
        .build();

    println!("{:?}", nls.chars);
}
