pub mod charset;

use self::charset::{CharSet, EN_CONSONANTS, EN_VOWELS};
use crate::pest::{error::Error, Parser};

use super::parsing::{QatParser, Rule};

#[derive(Debug)]
enum Node {
    Letter(char),
    Any,
    Vowel,
    Consonant,
}

#[derive(Debug)]
pub struct Pattern {
    nodes: Vec<Node>,
}

impl Pattern {
    pub fn new(source: &str) -> Result<Self, Error<Rule>> {
        let tree = QatParser::parse(Rule::pattern, source)?.next().unwrap();
        let nodes = tree
            .into_inner()
            .map(|p| match p.as_rule() {
                Rule::letter => Node::Letter(p.as_str().chars().next().unwrap()),
                Rule::dot => Node::Any,
                Rule::vowel => Node::Vowel,
                Rule::consonant => Node::Consonant,
                r => panic!("unrecognized rule {:?}", r),
            })
            .collect();
        Ok(Self { nodes })
    }

    pub fn matches(&self, word: &str) -> bool {
        self.nodes
            .iter()
            .zip(word.chars())
            .all(|(node, w)| match node {
                Node::Letter(c) => w == *c,
                Node::Vowel => EN_VOWELS.contains(w),
                Node::Consonant => EN_CONSONANTS.contains(w),
                Node::Any => true,
            })
    }
}
