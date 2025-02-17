use wordqat::SimplePattern;

use proptest::prelude::*;

#[derive(Debug, Clone)]
enum ExpressionPart {
    Letter(char),
    Dot(char),
    Set(char, u32),
    NegSet(char, u32),
}

fn as_bit(letter: char) -> u32 {
    1 << (letter as u32 - 'a' as u32)
}

fn bitset_as_runs(bitset: u32) -> Vec<Vec<char>> {
    let mut runs = Vec::new();
    let mut matches = 0;

    for c in 'a'..='z' {
        if bitset & as_bit(c) > 0 {
            if matches == 0 {
                runs.push(vec![c]);
            } else {
                runs.last_mut().unwrap().push(c);
            }
            matches += 1;
        } else {
            matches = 0;
        }
    }

    runs
}

fn serialize_runs(runs: Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for run in runs {
        if run.len() > 2 {
            result.push(*run.first().unwrap());
            result.push('-');
            result.push(*run.last().unwrap());
        } else {
            for c in run {
                result.push(c);
            }
        }
    }
    result
}

fn split_ex(part: ExpressionPart) -> (String, String) {
    match part {
        ExpressionPart::Letter(c) => (c.to_string(), c.to_string()),
        ExpressionPart::Dot(c) => (".".to_string(), c.to_string()),
        ExpressionPart::Set(c, bitset) => {
            let runs = bitset_as_runs(bitset | as_bit(c));
            let mut pattern = String::from('[');
            pattern.push_str(&serialize_runs(runs));
            pattern.push(']');
            (pattern, c.to_string())
        }
        ExpressionPart::NegSet(c, bitset) => {
            let runs = bitset_as_runs(bitset & !as_bit(c));
            let mut pattern = String::from("[!");
            pattern.push_str(&serialize_runs(runs));
            pattern.push(']');
            (pattern, c.to_string())
        }
    }
}

fn expression_parts() -> impl Strategy<Value = ExpressionPart> {
    prop_oneof![
        prop::char::range('a', 'z').prop_map(ExpressionPart::Letter),
        prop::char::range('a', 'z').prop_map(ExpressionPart::Dot),
        (prop::char::range('a', 'z'), prop::bits::u32::ANY)
            .prop_map(|(a, b)| ExpressionPart::Set(a, b)),
        (prop::char::range('a', 'z'), prop::bits::u32::ANY)
            .prop_map(|(a, b)| ExpressionPart::NegSet(a, b)),
    ]
}

fn expressions() -> impl Strategy<Value = (String, String)> {
    proptest::collection::vec(expression_parts(), 0..32).prop_map(|vec| {
        let (mut ex, mut word) = (String::new(), String::new());
        for part in vec {
            let (exs, words) = split_ex(part);
            ex.push_str(&exs);
            word.push_str(&words);
        }
        (ex, word)
    })
}

proptest! {
    #[test]
    fn test_parse_doesnt_crash(s in "\\PC*") {
        let _ = SimplePattern::try_from(s.as_str());
    }

    #[test]
    fn test_matches_simple_expression((ex, word) in expressions()) {
        let pattern = SimplePattern::try_from(ex.as_str()).unwrap();
        prop_assert!(pattern.is_match(&word));
    }
}
