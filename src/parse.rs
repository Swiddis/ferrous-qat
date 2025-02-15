// See the grammar file in docs/qat_grammar.pest

use nom::{
    bytes::complete::tag, character::complete::one_of, combinator::opt, multi::many0,
    sequence::preceded, IResult, Parser,
};

use crate::pattern::{Element, SimplePattern};

// Any char that can be exactly mapped to an Element
const UNIT_ELEMENTS: &str = "abcdefghijklmnoqprstuvwxyz0123456789.@#*><";
const ANY_LETTER: Element = Element::Set(0x3ffffff);
const VOWEL: Element = Element::Set(0x104111);
const CONSONANT: Element = Element::Set(0x3efbeee);

fn element(input: &str) -> IResult<&str, Element> {
    let (rem, c) = one_of(UNIT_ELEMENTS).parse(input)?;
    let elem = match c {
        'a'..='z' => Element::Set(1 << (c as u32 - 'a' as u32)),
        '0'..='9' => Element::Copy(c as u8),
        '.' => ANY_LETTER,
        '@' => VOWEL,
        '#' => CONSONANT,
        '*' => Element::Wildcard,
        '>' => Element::Word,
        '<' => Element::RevWord,
        _ => unreachable!(),
    };
    Ok((rem, elem))
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
