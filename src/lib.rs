use std::error::Error;

pub struct Pattern {}

impl Pattern {
    pub fn new(_qat: &str) -> Result<Self, Box<dyn Error>> {
        unimplemented!()
    }

    pub fn is_match(&self, _haystack: &str) -> bool {
        unimplemented!()
    }
}
