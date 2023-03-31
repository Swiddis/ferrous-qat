use wordqat::pattern::Pattern;

#[test]
fn test_matches_crossword() {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = Pattern::new("l..e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["lone", "love"])
}

#[test]
fn test_matches_set() {
    let wordlist = ["anise", "avize", "alone", "elide", "risen"];
    let pattern = Pattern::new("..i[sz]e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["anise", "avize"])
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
