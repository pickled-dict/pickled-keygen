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
