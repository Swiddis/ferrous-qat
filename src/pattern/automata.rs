use pest::iterators::{Pair, Pairs};

use crate::parsing::Rule;

use super::charset::{CharSet, EnCharSet};

#[derive(Debug)]
pub struct PatternNode {
    pub matches: EnCharSet,
    pub out: Vec<Box<PatternNode>>,
    pub is_terminal: bool,
}

impl PatternNode {
    fn make_set(pair: Pair<Rule>) -> EnCharSet {
        let mut set = EnCharSet::new();
        pair.into_inner().for_each(|p| match p.as_rule() {
            Rule::letter => set.insert(p.as_str().chars().next().unwrap()),
            Rule::set_range => {
                let mut chars = p.as_str().chars();
                let left = chars.next().expect("could not parse set range");
                let right = chars.nth(1).expect("could not parse set range");
                for c in left..=right {
                    set.insert(c);
                }
            }
            r => panic!("invalid set contents {:?}", r),
        });
        set
    }

    pub fn new(mut tree: Pairs<'_, Rule>) -> Option<Self> {
        let pair = tree.next()?;
        let follow = Self::new(tree);
        let matches = match pair.as_rule() {
            Rule::letter => EnCharSet::from_iter(pair.as_str().chars()),
            Rule::dot => EnCharSet::any(),
            Rule::vowel => EnCharSet::vowels(),
            Rule::consonant => EnCharSet::consonants(),
            Rule::set => Self::make_set(pair),
            Rule::negset => Self::make_set(pair).negate(),
            r => panic!("unrecognized rule {:?}", r),
        };
        match follow {
            None => Some(Self {
                matches,
                out: vec![],
                is_terminal: true,
            }),
            Some(anode) => Some(Self {
                matches,
                out: vec![Box::new(anode)],
                is_terminal: false,
            }),
        }
    }

    pub fn test(&self, word: &str, idx: usize) -> bool {
        if idx + 1 >= word.len() {
            if idx == word.len() {
                return self.is_terminal;
            }
            return self.is_terminal && self.matches.contains(word.chars().nth(idx).unwrap());
        }
        // TODO terrible indexing method for now, fix later
        if !self.matches.contains(word.chars().nth(idx).unwrap()) {
            return false;
        }
        if self.is_terminal && idx + 1 == word.len() {
            return true;
        }
        self.out.iter().any(|n| n.test(word, idx + 1))
    }
}
