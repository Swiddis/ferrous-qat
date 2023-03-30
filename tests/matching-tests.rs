use wordqat::Pattern;

#[test]
fn test_matches_crossword() {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = Pattern::new("l..e");
    let mut result = pattern.filter(&wordlist);
    result.sort();
    assert_eq!(result, vec!["lone", "love"])
}
