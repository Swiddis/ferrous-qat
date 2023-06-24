use proptest::prelude::*;
use wordqat::pattern::Pattern;

proptest! {
    #[test]
    fn test_compilation_no_panic(s in ".*") {
        let pattern = Pattern::new(s.as_str());
        assert!(pattern.is_ok() || pattern.is_err());
    }

    #[test]
    fn test_not_matches_non_unicode(s in "[^a-z]") {
        let pattern = Pattern::new(".").unwrap();
        assert_eq!(pattern.matches(s.as_str()), false);
    }

    #[test]
    fn test_matches_fixed(s in "[a-z]*") {
        let pattern = Pattern::new(s.as_str()).unwrap();
        assert_eq!(pattern.matches(s.as_str()), true);
    }

    #[test]
    fn test_matches_crossword_positive(a in "[a-z]*", b in "[a-z]", c in "[a-z]*") {
        let pattern = ".".repeat(a.len()) + &b + &".".repeat(c.len());
        let pattern = Pattern::new(&pattern).unwrap();
        assert_eq!(pattern.matches(&(a + &b + &c)), true);
    }

    #[test]
    fn test_not_matches_crossword_wrong_letters(a in "[a-z]", b in "[a-z]", c in "[a-z]*", d in "[a-z]*") {
        prop_assume!(a != b);
        let pattern = ".".repeat(c.len()) + &a + &".".repeat(d.len());
        let pattern = Pattern::new(&pattern).unwrap();
        assert_eq!(pattern.matches(&(c + &b + &d)), false);
    }

    #[test]
    fn test_not_matches_crossword_wrong_length(a in "[\\.a-z]*", b in "[a-z]*") {
        prop_assume!(a.len() != b.len());
        let pattern = Pattern::new(&a).unwrap();
        assert_eq!(pattern.matches(&b), false);
    }

    #[test]
    fn test_matches_set_positive(s in "[a-z]{2}i[sz]e") {
        let pattern = Pattern::new("..i[sz]e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), true);
    }

    #[test]
    fn test_not_matches_set_miss(s in "[a-z]{2}i[a-rt-y]e") {
        let pattern = Pattern::new("..i[sz]e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), false);
    }

    #[test]
    fn test_matches_negset_positive(s in "[a-z]{2}i[a-rt-y]e") {
        let pattern = Pattern::new("..i[!sz]e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), true);
    }

    #[test]
    fn test_not_matches_negset_miss(s in "[a-z]{2}i[sz]e") {
        let pattern = Pattern::new("..i[!sz]e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), false);
    }

    #[test]
    fn test_matches_set_range_positive(s in "[l-p][a-z][m-r][a-z][w-z]") {
        let pattern = Pattern::new("[l-p].[m-r].[w-z]").unwrap();
        assert_eq!(pattern.matches(s.as_str()), true);
    }

    #[test]
    fn test_not_matches_set_range_miss(s in "([a-kq-z][a-z]{4}|[a-z]{2}[a-ls-z][a-z]{2}|[a-z]{4}[a-v])") {
        let pattern = Pattern::new("[l-p].[m-r].[w-z]").unwrap();
        assert_eq!(pattern.matches(s.as_str()), false);
    }
}

#[test]
fn test_matches_vowels() {
    let wordlist = ["wine", "wins", "wise", "wore", "worn"];
    let pattern = Pattern::new("w@.@").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["wine", "wise", "wore"]);
}

#[test]
fn test_matches_consonants() {
    let wordlist = ["wine", "wins", "wise", "wore", "worn"];
    let pattern = Pattern::new("w..#").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["wins", "worn"]);
}
