// See the grammar file in docs/qat_grammar.pest

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{into, opt},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

use crate::pattern::{Element, LetterSet, SimplePattern, ANY_LETTER};

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
/// Any char that can be exactly mapped to an Element
const UNIT_ELEMENTS: &str = "abcdefghijklmnopqrstuvwxyz0123456789.@#*><";

fn unit_element(input: &str) -> IResult<&str, Element> {
    into(one_of(UNIT_ELEMENTS)).parse(input)
}

fn set_letter(input: &str) -> IResult<&str, LetterSet> {
    into(one_of(LOWERCASE)).parse(input)
}

fn set_range(input: &str) -> IResult<&str, LetterSet> {
    into(separated_pair(
        one_of(LOWERCASE),
        tag("-"),
        one_of(LOWERCASE),
    ))
    .parse(input)
}

fn set(input: &str) -> IResult<&str, Element> {
    let (rem, (negation, subsets)) = delimited(
        char('['),
        (opt(char('!')), many0(alt((set_letter, set_range)))),
        char(']'),
    )
    .parse(input)?;

    let reduced = subsets.into_iter().reduce(|a, b| LetterSet(a.0 | b.0));

    match (negation, reduced) {
        (None, None) => Ok((rem, Element::Set(LetterSet(0)))),
        (None, Some(s)) => Ok((rem, Element::Set(s))),
        (Some(_), None) => Ok((rem, Element::Set(ANY_LETTER))),
        (Some(_), Some(s)) => Ok((rem, Element::Set(LetterSet(ANY_LETTER.0 & !s.0)))),
    }
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((unit_element, set)).parse(input)
}

pub fn simple_pattern(input: &str) -> IResult<&str, SimplePattern> {
    let (rem, sequence) = many0(element).parse(input)?;
    let (rem, slash_sequence) = opt(preceded(tag("/"), many0(element))).parse(rem)?;

    Ok((
        rem,
        SimplePattern {
            sequence,
            slash_sequence,
        },
    ))
}
