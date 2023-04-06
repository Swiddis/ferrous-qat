pub mod charset;

use crate::pest::{Parser, error::Error};
use super::parsing::{QatParser, Rule};

#[derive(Debug)]
enum Node {
    Letter(char),
    Any,

}

#[derive(Debug)]
pub struct Pattern {
    nodes: Vec<Node>
}

impl Pattern {
    pub fn new(source: &str) -> Result<Self, Error<Rule>> {
        let tree = QatParser::parse(Rule::pattern, source)?.next().unwrap();
        let nodes = tree.into_inner().map(|p| match p.as_rule() {
            Rule::letter => Node::Letter(p.as_str().chars().next().unwrap()),
            Rule::dot => Node::Any,
            r => panic!("unrecognized rule {:?}", r)
        }).collect();
        Ok(Self { nodes })
    }

    pub fn matches(&self, word: &str) -> bool {
        for (node, w) in self.nodes.iter().zip(word.chars()) {
            match node {
                Node::Letter(c) => if w != *c {
                    return false;
                },
                Node::Any => continue
            }
        }
        return true;
    }
}
