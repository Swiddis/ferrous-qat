#[derive(Debug)]
pub enum Element {
    /// A single-character letter matcher as a bitset. Can flexibly handle
    /// 'a'..'z', '.', '@', '#', and '[...]'.
    Set(u32),
    /// Represents hooks to match the repeated decimal digit variables '0'..'9'.
    Copy(u8),
    /// Represents '*'.
    Wildcard,
    /// Represents '>'.
    Word,
    /// Represents '<'.
    RevWord,
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
                    if 1 << (c as u32 - 'a' as u32) & m == 0 {
                        return false;
                    }
                }
                _ => unimplemented!(),
            }
        }
        true
    }
}
