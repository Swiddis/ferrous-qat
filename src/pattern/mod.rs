pub mod charset;

use pest::iterators::Pair;

use self::charset::{CharSet, EnCharSet, EN_CONSONANTS, EN_VOWELS};
use crate::pest::{error::Error, Parser};

use super::parsing::{QatParser, Rule};

#[derive(Debug)]
enum Node {
    Letter(char),
    Set(EnCharSet),
    NegSet(EnCharSet),
    Any,
    Vowel,
    Consonant,
}

#[derive(Debug)]
pub struct Pattern {
    nodes: Vec<Node>,
}

impl Pattern {
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

    pub fn new(source: &str) -> Result<Self, Box<Error<Rule>>> {
        let tree = QatParser::parse(Rule::pattern, source)?.next().unwrap();
        let nodes = tree
            .into_inner()
            .map(|p| match p.as_rule() {
                Rule::letter => Node::Letter(p.as_str().chars().next().unwrap()),
                Rule::dot => Node::Any,
                Rule::vowel => Node::Vowel,
                Rule::consonant => Node::Consonant,
                Rule::set => Node::Set(Self::make_set(p)),
                Rule::negset => Node::NegSet(Self::make_set(p)),
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
                Node::Set(s) => s.contains(w),
                Node::NegSet(s) => !s.contains(w),
                Node::Vowel => EN_VOWELS.contains(w),
                Node::Consonant => EN_CONSONANTS.contains(w),
                Node::Any => true,
            })
    }
}
