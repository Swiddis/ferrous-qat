pub mod automata;
pub mod charset;

use self::{automata::PatternNode, charset::{CharSet, EnCharSet}};
use crate::pest::{error::Error, Parser};

use super::parsing::{QatParser, Rule};

#[derive(Debug)]
pub struct Pattern {
    node: PatternNode,
}

impl Pattern {
    pub fn new(source: &str) -> Result<Self, Box<Error<Rule>>> {
        let tree = QatParser::parse(Rule::pattern, source)?.next().unwrap();
        let node = PatternNode::new(tree.into_inner());
        match node {
            Some(n) => Ok(Self { node: n }),
            None => Ok(Self { node: PatternNode { matches: EnCharSet::new(), out: vec![], is_terminal: true }})
        }
    }

    pub fn matches(&self, word: &str) -> bool {
        self.node.test(word, 0)
    }
}
