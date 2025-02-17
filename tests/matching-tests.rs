use wordqat::SimplePattern;

use proptest::prelude::*;

#[derive(Debug, Clone)]
enum ExpressionPart {
    Letter(char),
    Dot(char),
    // Set(char, u32),
}

fn expression_part_strategy() -> impl Strategy<Value = ExpressionPart> {
    prop_oneof![
        prop::char::range('a', 'z').prop_map(ExpressionPart::Letter),
        prop::char::range('a', 'z').prop_map(ExpressionPart::Dot),
        // (prop::char::range('a', 'z'), prop::bits::u32::ANY).prop_map(|(a, b)| ExpressionPart::Set(a, b)),
    ]
}

fn split_ex(part: ExpressionPart) -> (String, String) {
    match part {
        ExpressionPart::Letter(c) => (c.to_string(), c.to_string()),
        ExpressionPart::Dot(c) => (".".to_string(), c.to_string()),
        // ExpressionPart::Set(c, m) => todo!(),
    }
}

proptest! {
    #[test]
    fn test_parse_doesnt_crash(s in "\\PC*") {
        let _ = SimplePattern::try_from(s.as_str());
    }

    #[test]
    fn test_matches_simple_expression(ex_parts in proptest::collection::vec(expression_part_strategy(), 0..50)) {
        let (mut ex, mut word) = (String::new(), String::new());
        for part in ex_parts {
            let (exs, words) = split_ex(part);
            ex.push_str(&exs);
            word.push_str(&words);
        }

        let pattern = SimplePattern::try_from(ex.as_str()).unwrap();
        prop_assert!(pattern.is_match(&word));
    }
}

#[test]
fn test_matches_set() {
    let wordlist = ["anise", "avize", "alone", "elide", "risen"];
    let pattern = SimplePattern::try_from("..i[sz]e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.is_match(w))
        .collect();
    assert_eq!(result, vec!["anise", "avize"])
}

#[test]
fn test_matches_negset() {
    let wordlist = ["anise", "avize", "alice", "taire"];
    let pattern = SimplePattern::try_from("..i[!sz]e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.is_match(w))
        .collect();
    assert_eq!(result, vec!["alice", "taire"]);
}
