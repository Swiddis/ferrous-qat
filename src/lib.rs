pub struct Pattern<'a> {
    pattern: &'a str,
}

impl<'a, 'b> Pattern<'a> {
    pub fn new(pattern: &'a str) -> Self {
        assert!(pattern.chars().all(|x| x.is_ascii_lowercase() || x == '.'));
        Self { pattern }
    }

    pub fn matches(&self, word: &str) -> bool {
        if self.pattern.len() != word.len() {
            return false;
        }
        for (a, b) in self.pattern.chars().zip(word.chars()) {
            match (a, a == b) {
                ('.', _) => continue,
                (_, true) => continue,
                (_, false) => return false,
            }
        }
        true
    }

    pub fn filter(&self, wordlist: &[&'b str]) -> Vec<&'b str> {
        wordlist
            .iter()
            .filter(|&x| self.matches(x))
            .copied()
            .collect()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_crossword() {
        let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
        let pattern = Pattern::new("l..e");
        let mut result = pattern.filter(&wordlist);
        result.sort();
        assert_eq!(result, vec!["lone", "love"])
    }
}
