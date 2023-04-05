pub mod charset;
pub mod parsing;

use self::charset::*;
use self::parsing::{ParsingError, Token};

enum Node {
    Any,
    Char(char),
    Set(EnCharSet),
    NegSet(EnCharSet),
    AnyVow,
    AnyCon,
}

pub struct Pattern {
    nodes: Vec<Node>,
}

impl<'a, 'b> Pattern {
    fn collect_set<I>(tokens: &mut I) -> Result<EnCharSet, ParsingError<'b>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut set = EnCharSet::new();
        for token in tokens.by_ref() {
            match token {
                Token::Letter(c) => set.insert(*c),
                Token::EndSet => return Ok(set),
                _ => {
                    return Err(ParsingError::SyntaxError("illegal set element"));
                }
            }
        }
        Err(ParsingError::SyntaxError("set not closed"))
    }

    pub fn new(source: &str) -> Result<Self, ParsingError> {
        let tokens = Token::lexify(source)?;
        let mut nodes = Vec::new();
        let mut tokens = tokens.iter().peekable();
        while let Some(token) = tokens.next() {
            match token {
                Token::Letter(c) => nodes.push(Node::Char(*c)),
                Token::AnyLetter => nodes.push(Node::Any),
                Token::AnyVowel => nodes.push(Node::AnyVow),
                Token::AnyConsonant => nodes.push(Node::AnyCon),
                Token::BeginSet => nodes.push(Node::Set(Self::collect_set(&mut tokens)?)),
                Token::BeginNegSet => nodes.push(Node::NegSet(Self::collect_set(&mut tokens)?)),
                Token::EndSet => {
                    return Err(ParsingError::SyntaxError("closed set without open"));
                }
            }
        }
        Ok(Self { nodes })
    }

    pub fn matches(&self, word: &str) -> bool {
        if self.nodes.len() != word.len() {
            return false;
        }
        self.nodes.iter().zip(word.chars()).all(|(n, w)| match n {
            Node::Any => true,
            Node::Char(c) => *c == w,
            Node::Set(s) => s.contains(w),
            Node::NegSet(s) => !s.contains(w),
            Node::AnyVow => {
                let vowels = EnCharSet::from_mask(EN_VOWELS);
                vowels.contains(w)
            }
            Node::AnyCon => {
                let consonants = EnCharSet::from_mask(EN_CONSONANTS);
                consonants.contains(w)
            }
        })
    }
}
