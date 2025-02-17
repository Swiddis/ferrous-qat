use std::collections::BTreeMap;

#[derive(Debug)]
struct Node {
    eof: bool,
    links: BTreeMap<char, usize>,
}

#[derive(Debug)]
pub struct Index {
    nodes: Vec<Node>
}

impl Index {
    pub fn new(words: Vec<&str>) -> Self {
        let mut nodes = vec![Node { eof: false, links: BTreeMap::new() }];
        for word in words {
            let mut state = 0;
            for c in word.chars() {
                match nodes[state].links.get(&c) {
                    Some(next) => state = *next,
                    None => {
                        let next = nodes.len();
                        nodes[state].links.insert(c, next);
                        nodes.push(Node { eof: false, links: BTreeMap::new() });
                        state = next;
                    }
                }
            }
            nodes[state].eof = true;
        }

        Self { nodes }
    }
}
