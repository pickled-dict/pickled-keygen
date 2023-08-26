pub struct AvailableChars {
    pub chars: Vec<char>,
}

impl AvailableChars {
    pub fn builder() -> AvailableCharsBuilder {
        AvailableCharsBuilder::default()
    }
}

#[derive(Default)]
pub struct AvailableCharsBuilder {
    chars: Vec<char>,
}

impl AvailableCharsBuilder {
    pub fn new() -> AvailableCharsBuilder {
        AvailableCharsBuilder { chars: vec![] }
    }

    pub fn symbols(mut self) -> AvailableCharsBuilder {
        let symbols = [
            '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
            '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
        ];

        symbols.iter().for_each(|&s| {
            self.chars.push(s);
        });

        self
    }

    pub fn numbers(mut self) -> AvailableCharsBuilder {
        let mut numbers: [char; 10] = ['a'; 10];

        for (i, ch) in (b'0'..=b'9').enumerate() {
            numbers[i] = char::from(ch);
        }

        numbers.iter().for_each(|&s| {
            self.chars.push(s);
        });

        self
    }

    pub fn lowercase(mut self) -> AvailableCharsBuilder {
        let mut lower: [char; 26] = ['a'; 26];

        for (i, ch) in (b'a'..=b'z').enumerate() {
            lower[i] = char::from(ch);
        }

        lower.iter().for_each(|&s| {
            self.chars.push(s);
        });

        self
    }

    pub fn uppercase(mut self) -> AvailableCharsBuilder {
        let mut upper: [char; 26] = ['a'; 26];

        for (i, ch) in (b'A'..=b'Z').enumerate() {
            upper[i] = char::from(ch);
        }

        upper.iter().for_each(|&s| {
            self.chars.push(s);
        });

        self
    }

    pub fn build(self) -> AvailableChars {
        AvailableChars { chars: self.chars }
    }
}
