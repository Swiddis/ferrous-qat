use logos::Logos;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParsingError<'a> {
    #[error("invalid token")]
    InvalidTokenError(),
    #[error("syntax error: {0}")]
    SyntaxError(&'a str),
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    #[regex("[a-z]+")]
    Letters(&'a str),
    #[token(".")]
    AnyLetter,
    #[token("[")]
    BeginSet,
    #[token("[!")]
    BeginNegSet,
    #[token("]")]
    EndSet,
    #[token("@")]
    AnyVowel,
    #[token("#")]
    AnyConsonant,
    #[regex("[a-z]-[a-z]")]
    SetRange(&'a str),
    #[error]
    Error,
}

#[cfg(test)]
mod test {
    use super::Token;
    use logos::Logos;

    #[test]
    fn test_lexify_letters() {
        use Token::*;
        let result = Token::lexer("abc").collect::<Vec<Token>>();
        assert_eq!(result, vec![Letters("abc")]);
    }

    #[test]
    fn test_lexify_set() {
        use Token::*;
        let result = Token::lexer("[ab]").collect::<Vec<Token>>();
        assert_eq!(result, vec![BeginSet, Letters("ab"), EndSet]);
    }

    #[test]
    fn test_lexify_negset() {
        use Token::*;
        let result = Token::lexer("[!ab]").collect::<Vec<Token>>();
        assert_eq!(result, vec![BeginNegSet, Letters("ab"), EndSet]);
    }

    #[test]
    fn test_lexify_vowels() {
        use Token::*;
        let result = Token::lexer("a@b").collect::<Vec<Token>>();
        assert_eq!(result, vec![Letters("a"), AnyVowel, Letters("b")]);
    }

    #[test]
    fn test_lexify_consonants() {
        use Token::*;
        let result = Token::lexer("a#b").collect::<Vec<Token>>();
        assert_eq!(result, vec![Letters("a"), AnyConsonant, Letters("b")]);
    }

    #[test]
    fn test_lexify_set_range() {
        use Token::*;
        let result = Token::lexer("[a-b]").collect::<Vec<Token>>();
        assert_eq!(result, vec![BeginSet, SetRange("a-b"), EndSet]);
    }
}
