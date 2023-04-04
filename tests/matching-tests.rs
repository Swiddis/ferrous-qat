use wordqat::pattern::Pattern;

#[test]
fn test_matches_crossword() {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = Pattern::new("l..e").unwrap();
    let result: Vec<&str> = wordlist
        .into_iter()
        .filter(|w| pattern.matches(w))
        .collect();
    assert_eq!(result, vec!["lone", "love"]);
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
