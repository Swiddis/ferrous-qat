use wordqat::Pattern;

#[test]
fn test_matches_crossword() {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = Pattern::new("l..e").unwrap();
    let mut result = pattern.filter(&wordlist);
    result.sort();
    assert_eq!(result, vec!["lone", "love"])
}

#[test]
fn test_matches_set() {
    let wordlist = ["anise", "avize", "alone", "elide", "risen"];
    let pattern = Pattern::new("..i[sz]e").unwrap();
    let mut result = pattern.filter(&wordlist);
    result.sort();
    assert_eq!(result, vec!["anise", "avize"])
}
