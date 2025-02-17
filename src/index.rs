use trie_rs::{Trie, TrieBuilder};

use crate::{pattern::Element, SimplePattern};

#[derive(Debug)]
pub struct Index {
    trie: Trie<u8>
}

impl Index {
    pub fn new(words: Vec<&str>) -> Self {
        Self { trie: words.into_iter().collect() }
    }

    pub fn search(&self, pattern: &SimplePattern) -> Vec<&str> {
        let mut results = Vec::new();
        let mut inc_search = self.trie.inc_search();

        for elem in pattern.sequence.iter() {
            match elem {
                Element::Set(mask) => {},
                _ => todo!(),
            }
        }

        results
    }
}
