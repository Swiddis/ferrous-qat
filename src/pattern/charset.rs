pub trait CharSet {
    fn new() -> Self;
    fn insert(&mut self, c: char);
    fn contains(&self, c: char) -> bool;
}

pub struct EnCharSet {
    bits: u32,
}

impl EnCharSet {
    fn as_bit(c: char) -> u32 {
        debug_assert!(c.is_ascii_lowercase());
        1 << (c as u32 - 'a' as u32)
    }
}

impl CharSet for EnCharSet {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn insert(&mut self, c: char) {
        self.bits |= Self::as_bit(c);
    }

    fn contains(&self, c: char) -> bool {
        if !c.is_ascii_lowercase() {
            return false;
        }
        Self::as_bit(c) & self.bits > 0
    }
}

impl FromIterator<char> for EnCharSet {
    fn from_iter<T>(chars: T) -> Self
    where
        T: IntoIterator<Item = char>,
    {
        let mut bits: u32 = 0;
        for c in chars.into_iter() {
            bits |= Self::as_bit(c);
        }
        Self { bits }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encharset_new() {
        let test_chars = "asdf";
        let mut set = EnCharSet::new();
        for c in test_chars.chars() {
            set.insert(c);
        }
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(set.contains(c), test_chars.contains(c));
        }
    }

    #[test]
    fn test_encharset_from_iter() {
        let test_chars = "asdf";
        let set = EnCharSet::from_iter(test_chars.chars());
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(set.contains(c), test_chars.contains(c));
        }
    }
}
