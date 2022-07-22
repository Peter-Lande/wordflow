use std::ops::Add;

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

        return Token::Text(
            spaced_text
                .split_whitespace()
                .map(|string_reference| string_reference.to_owned())
                .collect::<Vec<String>>()
                .into_boxed_slice(),
        );
    }
}

impl From<&str> for Token {
    fn from(input: &str) -> crate::tokenization::Token {
        Self::from(String::from(input))
    }
}

impl Add for Token {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Token::None => match other {
                Token::None => return Token::None,
                Token::Text(text) => return Token::Text(text),
            },
            Token::Text(text_self) => match other {
                Token::None => return Token::Text(text_self),
                Token::Text(text_other) => {
                    return Token::Text(
                        (*text_self)
                            .into_iter()
                            .chain((*text_other).into_iter())
                            .map(|string_reference| string_reference.to_owned())
                            .collect::<Vec<String>>()
                            .into_boxed_slice(),
                    )
                }
            },
        }
    }
}
