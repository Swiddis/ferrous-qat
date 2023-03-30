use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("index {0}: {1}")]
    InvalidTokenError(usize, char),
    #[error("index {0}: {1}")]
    SyntaxError(usize, String),
}

enum Token {
    Letter(char),
    Set(Vec<char>),
    Any,
    Empty, // Dummy value for parsing, always optimized away
}

struct ParseState {
    make_set: bool,
    set: Vec<char>,
}

pub struct Pattern {
    tokens: Vec<Token>,
}

impl<'a> Pattern {
    pub fn new(raw_pattern: &str) -> Result<Self, ParsingError> {
        let mut state = ParseState {
            make_set: false,
            set: Vec::new(),
        };
        let tokens = raw_pattern
            .chars()
            .enumerate()
            .map(|(i, c)| match (c, state.make_set) {
                ('.', false) => Ok(Token::Any),
                ('.', true) => Err(ParsingError::SyntaxError(i, "wildcard in set".to_owned())),
                ('[', false) => {
                    state.make_set = true;
                    state.set = Vec::new();
                    Ok(Token::Empty)
                }
                ('[', true) => Err(ParsingError::SyntaxError(i, "set opened twice".to_owned())),
                (']', false) => Err(ParsingError::SyntaxError(
                    i,
                    "set closed before open".to_owned(),
                )),
                (']', true) => {
                    let out = Ok(Token::Set(state.set.clone()));
                    state.make_set = false;
                    state.set = Vec::new();
                    out
                }
                ('a'..='z', false) => Ok(Token::Letter(c)),
                ('a'..='z', true) => {
                    state.set.push(c);
                    Ok(Token::Empty)
                }
                _ => Err(ParsingError::InvalidTokenError(i, c)),
            })
            .filter(|t| !matches!(t, Ok(Token::Empty)))
            .collect::<Result<Vec<Token>, ParsingError>>()?;
        Ok(Self { tokens })
    }

    pub fn matches(&self, word: &str) -> bool {
        if self.tokens.len() != word.len() {
            return false;
        }
        self.tokens.iter().zip(word.chars()).all(|(t, w)| match t {
            Token::Letter(c) => *c == w,
            Token::Set(s) => s.contains(&w),
            Token::Any => true,
            Token::Empty => panic!("empty token not cleared during parsing"),
        })
    }

    pub fn filter(&self, wordlist: &[&'a str]) -> Vec<&'a str> {
        wordlist
            .iter()
            .filter(|&x| self.matches(x))
            .copied()
            .collect()
    }
}
