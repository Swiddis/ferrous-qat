pub mod charset;
pub mod parsing;

use self::charset::{CharSet, EnCharSet};
use self::parsing::{ParsingError, Token};

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
        I: Iterator<Item = &'a Token>,
    {
        use ParsingError::SyntaxError;
        let mut set = EnCharSet::new();
        let (mut in_range, mut range_start) = (false, '\0');
        for token in tokens.by_ref() {
            match token {
                Token::Letter(c) => match (in_range, range_start) {
                    (false, _) => {
                        set.insert(*c);
                        range_start = *c;
                    }
                    (true, '\0') => return Err(SyntaxError("set range with no start")),
                    (true, start) => {
                        for r in start..=*c {
                            set.insert(r);
                        }
                        in_range = false;
                        range_start = '\0';
                    }
                },
                Token::EndSet => {
                    return if in_range {
                        Err(SyntaxError("set closed while in range"))
                    } else {
                        Ok(set)
                    }
                }
                Token::SetRange => {
                    in_range = true;
                }
                _ => {
                    return Err(SyntaxError("illegal set element"));
                }
            }
        }
        Err(SyntaxError("set not closed"))
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
                Token::SetRange => {
                    return Err(ParsingError::SyntaxError("set range not present in set"));
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
                let vowels = EnCharSet::from_iter("aeiou".chars());
                vowels.contains(w)
            }
            Node::AnyCon => {
                let consonants = EnCharSet::from_iter("bcdfghjklmnpqrstvwxyz".chars());
                consonants.contains(w)
            }
        })
    }
}
