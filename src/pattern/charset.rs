use core::fmt::Debug;

pub const EN_VOWELS: u32 = 0x00104111;
pub const EN_CONSONANTS: u32 = 0x03efbeee;

pub trait CharSet<T> {
    fn new() -> Self;
    fn from_mask(mask: T) -> Self;
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

impl CharSet<u32> for EnCharSet {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn from_mask(mask: u32) -> Self {
        Self { bits: mask }
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

impl Debug for EnCharSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnCharSet")
            .field("bits", &self.bits)
            .field(
                "_chars",
                &('a'..='z')
                    .filter(|c| self.contains(*c))
                    .collect::<Vec<char>>(),
            )
            .finish()
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
