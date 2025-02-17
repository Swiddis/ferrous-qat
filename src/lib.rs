mod parse;
mod pattern;
mod index;

pub use pattern::SimplePattern;
pub use index::Index;
use nom::{combinator::all_consuming, Parser};

impl TryFrom<&str> for SimplePattern {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match all_consuming(parse::simple_pattern).parse(value) {
            Ok((_, pattern)) => Ok(pattern),
            Err(e) => Err(e.to_string()),
        }
    }
}
