use std::collections::HashMap;

use crate::tokenization;

struct Ngram {
    data: HashMap<tokenization::Sentence, HashMap<tokenization::Token, usize>>,
    state_length: usize,
}

impl Ngram {
    pub fn new() -> Ngram {
        return Ngram {
            data: HashMap::new(),
            state_length: 2,
        };
    }
}
