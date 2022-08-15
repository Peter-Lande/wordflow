#![deny(missing_docs)]

use std::collections::HashMap;

use crate::tokenization;
struct Ngram {
    data: HashMap<tokenization::Sentence, HashMap<tokenization::Token, usize>>,
    state_length: usize,
}

impl Ngram {
    pub fn new(state_length: usize) -> Ngram {
        return Ngram {
            data: HashMap::new(),
            state_length: state_length,
        };
    }

    fn add_state(mut self, state: tokenization::Sentence) -> () {
        if self.state_length != state.len() {
            panic!("Sentence length is not equivalent to state_length of Ngram.");
        }
        self.data.insert(state, HashMap::new());
    }

    pub fn add_corpus(self, corpus: String) -> () {}
}
