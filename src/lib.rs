mod parse;
mod pattern;

pub use pattern::SimplePattern;

impl TryFrom<&str> for SimplePattern {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (rem, pattern) = parse::simple_pattern(value).unwrap();
        if rem.is_empty() {
            Ok(pattern)
        } else {
            Err(format!(
                "unrecognized character {}",
                rem.chars().next().unwrap()
            ))
        }
    }
}
