use proptest::prelude::*;
use wordqat::pattern::Pattern;

proptest! {
    #[test]
    fn test_matches_fixed(s in "[a-z]+") {
        let pattern = Pattern::new(s.as_str()).unwrap();
        assert_eq!(pattern.matches(s.as_str()), true);
    }

    #[test]
    fn test_matches_empty(s in "[a-z]*") {
        let pattern = Pattern::new("").unwrap();
        assert_eq!(pattern.matches(s.as_str()), s.as_str() == "");
    }

    #[test]
    fn test_matches_crossword_positive(s in "l[a-z]{2}e") {
        let pattern = Pattern::new("l..e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), true);
    }

    #[test]
    fn test_not_matches_crossword_wrong_letters(s in "([^l][a-z]{3}|[a-z]{3}[^e])") {
        let pattern = Pattern::new("l..e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), false);
    }

    #[test]
    fn test_not_matches_crossword_wrong_length(s in "([a-z]{0,3}|[a-z]{4,})") {
        let pattern = Pattern::new("l..e").unwrap();
        assert_eq!(pattern.matches(s.as_str()), false);
    }
}

#[test]
fn test_matches_set() {
    let wordlist = ["anise", "avize", "alone", "elide", "risen"];
    let pattern = Pattern::new("..i[sz]e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["anise", "avize"]);
}

#[test]
fn test_matches_negset() {
    let wordlist = ["anise", "avize", "alice", "taire"];
    let pattern = Pattern::new("..i[!sz]e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["alice", "taire"]);
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

#[test]
fn test_set_range() {
    let wordlist = ["lapaz", "parix", "other"];
    let pattern = Pattern::new("[l-p].[m-r].[w-z]").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["lapaz", "parix"]);
}
