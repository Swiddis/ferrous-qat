use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParsingError<'a> {
    #[error("invalid token at index {0}: {1}")]
    InvalidTokenError(usize, char),
    #[error("syntax error at index {0}: {1}")]
    SyntaxError(usize, &'a str),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Letter(char),
    AnyLetter,
    BeginSet,
    BeginNegSet,
    EndSet,
}

impl Token {
    pub fn lexify(source: &str) -> Result<Vec<Self>, ParsingError> {
        let mut tokens = Vec::new();
        let mut source = source.chars().enumerate().peekable();
        while let Some((idx, curr)) = source.next() {
            match curr {
                'a'..='z' => tokens.push(Self::Letter(curr)),
                '.' => tokens.push(Self::AnyLetter),
                '[' => match source.peek() {
                    Some(&(_, '!')) => {
                        tokens.push(Self::BeginNegSet);
                        source.next();
                    }
                    _ => tokens.push(Self::BeginSet),
                },
                ']' => tokens.push(Self::EndSet),
                _ => return Err(ParsingError::InvalidTokenError(idx, curr)),
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::Token;

    #[test]
    fn test_lexify_letters() {
        use Token::*;
        let result = Token::lexify("abc").unwrap();
        assert_eq!(result, vec![Letter('a'), Letter('b'), Letter('c')])
    }

    #[test]
    fn test_lexify_set() {
        use Token::*;
        let result = Token::lexify("[ab]").unwrap();
        assert_eq!(result, vec![BeginSet, Letter('a'), Letter('b'), EndSet]);
    }

    #[test]
    fn test_lexify_negset() {
        use Token::*;
        let result = Token::lexify("[!ab]").unwrap();
        assert_eq!(result, vec![BeginNegSet, Letter('a'), Letter('b'), EndSet]);
    }
}
