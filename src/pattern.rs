pub const ANY_LETTER: LetterSet = LetterSet(0x3ffffff);
const VOWELS: LetterSet = LetterSet(0x104111);
const CONSONANTS: LetterSet = LetterSet(0x3efbeee);

#[derive(Clone, Copy, Debug)]
pub struct LetterSet(pub u32);

impl From<char> for LetterSet {
    fn from(value: char) -> Self {
        assert!(value.is_ascii_lowercase());
        Self(1 << (value as u32 - 'a' as u32))
    }
}

impl From<(char, char)> for LetterSet {
    /// Parse the letter set as a range between the given letters
    fn from((left, right): (char, char)) -> Self {
        assert!(left.is_ascii_lowercase() && right.is_ascii_lowercase());
        let (left, right) = (
            std::cmp::min(left, right) as u32 - 'a' as u32,
            std::cmp::max(left, right) as u32 - 'a' as u32 + 1,
        );
        Self(ANY_LETTER.0 << left & !(ANY_LETTER.0 << right))
    }
}

#[derive(Debug)]
pub enum Element {
    /// A single-character letter matcher as a bitset. Can flexibly handle
    /// 'a'..'z', '.', '@', '#', and '[...]'.
    Set(LetterSet),
    /// Represents hooks to match the repeated decimal digit variables '0'..'9'.
    Copy(u8),
    /// Represents '*'.
    Wildcard,
    /// Represents '>'.
    Word,
    /// Represents '<'.
    RevWord,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            'a'..='z' => Self::Set(value.into()),
            '0'..='9' => Self::Copy(value as u8),
            '.' => Self::Set(ANY_LETTER),
            '@' => Self::Set(VOWELS),
            '#' => Self::Set(CONSONANTS),
            '*' => Self::Wildcard,
            '>' => Self::Word,
            '<' => Self::RevWord,
            _ => panic!("illegal Element character '{value}'"),
        }
    }
}

/// A SimplePattern can be resolved by traversing it from left to right.
#[derive(Debug)]
pub struct SimplePattern {
    pub sequence: Vec<Element>,
    pub slash_sequence: Option<Vec<Element>>,
}

impl SimplePattern {
    pub fn is_match(&self, word: &str) -> bool {
        for (elem, c) in self.sequence.iter().zip(word.chars()) {
            match elem {
                Element::Set(m) => {
                    if 1 << (c as u32 - 'a' as u32) & m.0 == 0 {
                        return false;
                    }
                }
                _ => unimplemented!(),
            }
        }
        true
    }
}
