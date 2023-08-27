use crate::available_chars::AvailableChars;
use rand::seq::SliceRandom;

pub fn generate_string(available: AvailableChars, size: usize) -> String {
    (0..size)
        .map(|_| available.chars.choose(&mut rand::thread_rng()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_with_default_builder_has_all_options() {
        let available = AvailableChars::builder().default_options().build();

        let size = 10;
        let result = generate_string(available, size);

        assert!(result.chars().all(|c| c.is_digit(10)
            || c.is_ascii_lowercase()
            || c.is_ascii_uppercase()
            || c.is_ascii_punctuation()));
    }

    #[test]
    fn test_generate_produces_the_correct_length() {
        let available = AvailableChars::builder().default_options().build();

        let size = 10;
        let result = generate_string(available, size);

        assert!(result.len() == size);
    }

    #[test]
    fn test_generate_with_zero_count_produces_the_correct_length_and_string() {
        let available = AvailableChars::builder().default_options().build();

        let size = 0;
        let result = generate_string(available, size);

        assert!(result.len() == 0);
        assert!(result == String::from(""));
    }

    #[test]
    fn test_generate_with_just_lowercase_builder_produces_correct_result() {
        let available = AvailableChars::builder().lowercase().build();

        let size = 10;
        let result = generate_string(available, size);

        assert!(result.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_generate_with_just_uppercase_builder_produces_correct_result() {
        let available = AvailableChars::builder().uppercase().build();

        let size = 10;
        let result = generate_string(available, size);

        assert!(result.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_generate_with_just_numbers_builder_produces_correct_result() {
        let available = AvailableChars::builder().numbers().build();

        let size = 10;
        let result = generate_string(available, size);

        assert!(result.chars().all(|c| c.is_digit(10)));
    }

    #[test]
    fn test_generate_with_just_symbols_builder_produces_correct_result() {
        let available = AvailableChars::builder().symbols().build();

        let size = 10;
        let result = generate_string(available, size);

        assert!(result.chars().all(|c| c.is_ascii_punctuation()));
    }
}
