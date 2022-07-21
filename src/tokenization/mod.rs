#[derive(PartialEq, Debug)]
pub enum Token {
    Text(Box<[String]>),
    None,
}

impl Token {}

impl From<String> for Token {
    fn from(input: String) -> crate::tokenization::Token {
        if input.is_empty() {
            return Token::None;
        }
        let mut spaced_text: String = input;
        for character in ['-', '_', '"', '?', '!', '.', ',', '(', ')', ';', '\'', ':'].iter() {
            spaced_text = spaced_text.replace(&character.to_string(), &format!(" {character} "));
        }
        let mut split_text: Vec<String> = Vec::new();
        for word in spaced_text.split_whitespace() {
            split_text.push(String::from(word));
        }
        return Token::Text(split_text.into_boxed_slice());
    }
}
