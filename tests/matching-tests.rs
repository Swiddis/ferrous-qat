use wordqat::SimplePattern;

#[test]
fn test_matches_crossword() {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = SimplePattern::try_from("l..e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.is_match(w))
        .collect();
    assert_eq!(result, vec!["lone", "love"])
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
