use std::cmp;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum Token {
    Text(String),
    None,
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::None => return 0,
            Token::Text(text) => return text.chars().count(),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Token::None => return String::from(""),
            Token::Text(text) => return text.to_string(),
        }
    }

    pub fn min_edit_distance(&self, other: &Token) -> usize {
        if self == other {
            return 0;
        }
        let self_length: usize = self.len();
        let other_length: usize = other.len();
        let mut edit_distance: Vec<Vec<usize>> = vec![vec![0; self_length + 1]; other_length + 1];
        for row_index in 1..self_length + 1 {
            edit_distance[0][row_index] = edit_distance[0][row_index - 1] + 1;
        }
        for column_index in 1..other_length + 1 {
            edit_distance[column_index][0] = edit_distance[column_index - 1][0] + 1;
        }
        for row_index in 1..other_length + 1 {
            for column_index in 1..self_length + 1 {
                let a: usize = if self.as_string().chars().nth(column_index - 1)
                    == other.as_string().chars().nth(row_index - 1)
                {
                    0
                } else {
                    2
                };
                edit_distance[row_index][column_index] = cmp::min(
                    edit_distance[row_index - 1][column_index] + 1,
                    cmp::min(
                        edit_distance[row_index][column_index - 1] + 1,
                        edit_distance[row_index - 1][column_index - 1] + a,
                    ),
                )
            }
        }
        return edit_distance[other_length][self_length];
    }
}

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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.as_string() == other.as_string();
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_string().hash(state);
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
        //We choose to use tokenize by keeping hypentated words together and separating out punctuation.
        for character in ['"', '?', '!', '.', ',', '(', ')', ';', ':', '$'].iter() {
            spaced_text = spaced_text.replace(&character.to_string(), &format!(" {character} "));
        }

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
