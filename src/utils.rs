use crate::available_chars::AvailableChars;
use rand::seq::SliceRandom;

pub fn generate_string(available: AvailableChars, size: usize) -> String {
    (0..size)
        .map(|_| available.chars.choose(&mut rand::thread_rng()).unwrap())
        .collect()
}
