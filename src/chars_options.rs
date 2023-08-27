#[derive(Copy, Clone)]
pub struct CharsOptions {
    pub upper: bool,
    pub lower: bool,
    pub numbers: bool,
    pub symbols: bool,
}

impl CharsOptions {
    pub fn is_empty(self) -> bool {
        self.numbers == false && self.lower == false && self.symbols == false && self.upper == false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_returns_true_if_empty() {
        let options = CharsOptions {
            upper: false,
            lower: false,
            numbers: false,
            symbols: false,
        };
        assert!(options.is_empty());
    }

    #[test]
    fn test_is_empty_returns_false_if_upper_is_true() {
        let options = CharsOptions {
            upper: true,
            lower: false,
            numbers: false,
            symbols: false,
        };
        assert!(!options.is_empty());
    }

    #[test]
    fn test_is_empty_returns_false_if_lower_is_true() {
        let options = CharsOptions {
            upper: false,
            lower: true,
            numbers: false,
            symbols: false,
        };
        assert!(!options.is_empty());
    }

    #[test]
    fn test_is_empty_returns_false_if_numbers_is_true() {
        let options = CharsOptions {
            upper: false,
            lower: false,
            numbers: true,
            symbols: false,
        };
        assert!(!options.is_empty());
    }

    #[test]
    fn test_is_empty_returns_false_if_symbols_is_true() {
        let options = CharsOptions {
            upper: false,
            lower: false,
            numbers: false,
            symbols: true,
        };
        assert!(!options.is_empty());
    }

    #[test]
    fn test_is_empty_returns_false_if_all_are_true() {
        let options = CharsOptions {
            upper: true,
            lower: true,
            numbers: true,
            symbols: true,
        };
        assert!(!options.is_empty());
    }
}
