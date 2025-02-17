use wordqat::SimplePattern;

use proptest::prelude::*;

#[derive(Debug, Clone)]
enum ExpressionPart {
    Letter(char),
    Dot(char),
    Set(char, u32),
    NegSet(char, u32),
}

fn expression_part_strategy() -> impl Strategy<Value = ExpressionPart> {
    prop_oneof![
        prop::char::range('a', 'z').prop_map(ExpressionPart::Letter),
        prop::char::range('a', 'z').prop_map(ExpressionPart::Dot),
        (prop::char::range('a', 'z'), prop::bits::u32::ANY).prop_map(|(a, b)| ExpressionPart::Set(a, b)),
        (prop::char::range('a', 'z'), prop::bits::u32::ANY).prop_map(|(a, b)| ExpressionPart::NegSet(a, b)),
    ]
}

fn split_ex(part: ExpressionPart) -> (String, String) {
    match part {
        ExpressionPart::Letter(c) => (c.to_string(), c.to_string()),
        ExpressionPart::Dot(c) => (".".to_string(), c.to_string()),
        ExpressionPart::Set(c, m) => {
            let mut pattern = String::from('[');
            for l in 'a'..='z' {
                if (1 << (l as u32 - 'a' as u32) & m) > 0 || l == c {
                    pattern.push(l);
                }
            }
            pattern.push(']');
            (pattern, c.to_string())
        },
        ExpressionPart::NegSet(c, m) => {
            let mut pattern = String::from("[!");
            for l in 'a'..='z' {
                if (1 << (l as u32 - 'a' as u32) & m) == 0 && l != c {
                    pattern.push(l);
                }
            }
            pattern.push(']');
            (pattern, c.to_string())
        },
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
        dbg!(&ex, &word);

        let pattern = SimplePattern::try_from(ex.as_str()).unwrap();
        prop_assert!(pattern.is_match(&word));
    }
}
