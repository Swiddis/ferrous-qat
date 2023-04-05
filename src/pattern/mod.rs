pub mod charset;
pub mod parsing;

use self::charset::*;
use self::parsing::{ParsingError, Token};
use logos::Logos;

#[derive(Debug)]
enum Node {
    Any,
    Char(char),
    Set(EnCharSet),
    NegSet(EnCharSet),
    AnyVow,
    AnyCon,
}

#[derive(Debug)]
pub struct Pattern {
    nodes: Vec<Node>,
}

impl<'a, 'b> Pattern {
    fn collect_set<I>(tokens: &mut I) -> Result<EnCharSet, ParsingError<'b>>
    where
        I: Iterator<Item = Token<'a>>,
    {
        use ParsingError::SyntaxError;
        let mut set = EnCharSet::new();
        for token in tokens.by_ref() {
            match token {
                Token::Letters(c) => {
                    for c in c.chars() {
                        set.insert(c);
                    }
                }
                Token::EndSet => {
                    return Ok(set);
                }
                Token::SetRange(r) => {
                    let mut c = r.chars();
                    // Length guaranteed to be 3
                    let (c0, c1) = (c.next().unwrap(), c.last().unwrap());
                    for c in c0..=c1 {
                        set.insert(c);
                    }
                }
                _ => {
                    return Err(SyntaxError("illegal set element"));
                }
            }
        }
        Err(SyntaxError("set not closed"))
    }

    pub fn new(source: &str) -> Result<Self, ParsingError> {
        let tokens = Token::lexer(source);
        let mut nodes = Vec::new();
        let mut tokens = tokens.peekable();
        while let Some(token) = tokens.next() {
            match token {
                Token::Letters(c) => {
                    for c in c.chars() {
                        nodes.push(Node::Char(c));
                    }
                }
                Token::AnyLetter => nodes.push(Node::Any),
                Token::AnyVowel => nodes.push(Node::AnyVow),
                Token::AnyConsonant => nodes.push(Node::AnyCon),
                Token::BeginSet => nodes.push(Node::Set(Self::collect_set(&mut tokens)?)),
                Token::BeginNegSet => nodes.push(Node::NegSet(Self::collect_set(&mut tokens)?)),
                Token::EndSet => {
                    return Err(ParsingError::SyntaxError("closed set without open"));
                }
                Token::SetRange(_) => {
                    return Err(ParsingError::SyntaxError("set range not present in set"));
                }
                Token::Error => {
                    return Err(ParsingError::InvalidTokenError());
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
