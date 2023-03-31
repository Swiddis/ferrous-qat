pub mod parsing;

use crate::pattern::parsing::{ParsingError, Token};

enum Node {
    Any,
    Char(char),
    Set(Vec<char>),
    NegSet(Vec<char>),
}

pub struct Pattern {
    nodes: Vec<Node>,
}

impl<'a, 'b> Pattern {
    fn collect_set<I>(tokens: &mut I) -> Result<Vec<char>, ParsingError<'b>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut set = vec![];
        for token in tokens.by_ref() {
            match token {
                Token::Letter(c) => set.push(*c),
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
            Node::Set(s) => s.contains(&w),
            Node::NegSet(s) => !s.contains(&w),
        })
    }
}
