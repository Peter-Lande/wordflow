pub enum Tokenized {
    Text(Box<[String]>),
    None,
}

impl Tokenized {
    fn from_string(input: String) -> crate::tokenization::Tokenized {
        if input.is_empty() {
            return Tokenized::None;
        }
        let mut split_text: Vec<String> = Vec::new();
        for word in input.split_whitespace() {
            for sub_string in word.split(&['-', '_', '"', '?', '!', '.', ',', '(', ')']) {
                split_text.push(String::from(sub_string));
            }
        }
        return Tokenized::Text(split_text.into_boxed_slice());
    }
}
