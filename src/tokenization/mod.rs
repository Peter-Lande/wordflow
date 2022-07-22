use std::ops::Add;
//TODO Make tokens a single string and add a structure that represents a slice of Tokens, maybe call it a sentence?

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Text(String),
    None,
}

impl Token {}

impl From<String> for Token {
    fn from(input: String) -> Token {
        if input.is_empty() {
            return Token::None;
        }
        return Token::Text(input);
    }
}

impl From<&str> for Token {
    fn from(input: &str) -> Token {
        Self::from(String::from(input))
    }
}

#[derive(PartialEq, Debug)]
pub enum Sentence {
    Text(Box<[Token]>),
    None,
}

impl Sentence {}

impl From<String> for Sentence {
    fn from(input: String) -> Sentence {
        if input.is_empty() {
            return Sentence::None;
        }
        let mut spaced_text: String = input;
        //We choose to use Penn Treebank Tokenization, which keeps hypentated words together, separates out punctuation, and separates clitics.
        for character in ['"', '?', '!', '.', ',', '(', ')', ';', ':', '$'].iter() {
            spaced_text = spaced_text.replace(&character.to_string(), &format!(" {character} "));
        }
        //TODO Separate out clitics.

        return Sentence::Text(
            spaced_text
                .split_whitespace()
                .map(|string_reference| Token::Text(string_reference.to_owned()))
                .collect::<Vec<Token>>()
                .into_boxed_slice(),
        );
    }
}

impl From<&str> for Sentence {
    fn from(input: &str) -> Sentence {
        Self::from(String::from(input))
    }
}

impl Add for Sentence {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Sentence::None => match other {
                Sentence::None => return Sentence::None,
                Sentence::Text(text) => return Sentence::Text(text),
            },
            Sentence::Text(text_self) => match other {
                Sentence::None => return Sentence::Text(text_self),
                Sentence::Text(text_other) => {
                    return Sentence::Text(
                        (*text_self)
                            .into_iter()
                            .chain((*text_other).into_iter())
                            .map(|string_reference| string_reference.to_owned())
                            .collect::<Vec<Token>>()
                            .into_boxed_slice(),
                    )
                }
            },
        }
    }
}
