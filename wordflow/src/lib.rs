#![deny(missing_docs)]
//! Wordflow:  A natural language processing library
//!

///The tokenization module contains objects and functions for tokenizing strings.
pub mod tokenization;

///The containers module contains structs for processing data.
pub mod containers;
#[cfg(test)]
mod tests {
    use crate::containers;
    use crate::tokenization::{Sentence, Token};

    #[test]
    fn test_from_string_token() {
        assert_eq!(Token::from(String::from("")), Token::None);
        assert_eq!(
            Token::from(String::from("Hello")),
            Token::Text(String::from("Hello"))
        );
    }

    #[test]
    fn test_from_str_token() {
        assert_eq!(Token::from(""), Token::None);
        assert_eq!(Token::from("Hello"), Token::Text(String::from("Hello")));
    }

    #[test]
    fn test_len_token() {
        assert_eq!(Token::None.len(), 0);
        assert_eq!(Token::from("Hello").len(), 5);
        assert_eq!(Token::from("").len(), 0);
    }

    #[test]
    fn test_as_string_token() {
        assert_eq!(Token::None.as_string(), String::from(""));
        assert_eq!(Token::from("Hello").as_string(), String::from("Hello"));
        assert_eq!(Token::from("").as_string(), String::from(""));
    }

    #[test]
    fn test_min_edit_distance_token() {
        assert_eq!(Token::None.min_edit_distance(&Token::None), 0);
        assert_eq!(
            Token::from("execution").min_edit_distance(&Token::from("intention")),
            8
        );
        assert_eq!(
            Token::from("Hello").min_edit_distance(&Token::from("Higher")),
            7
        );
        assert_eq!(
            Token::from("Hello").min_edit_distance(&Token::from("World")),
            8
        );

        assert_eq!(
            Token::from("There").min_edit_distance(&Token::from("Amazing")),
            12
        );
    }
    //End of Token tests

    #[test]
    fn test_from_string_sentence() {
        assert_eq!(Token::None, Token::from(String::from("")));
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("Hello"),
                Token::from("there"),
                Token::from("!")
            ])),
            Sentence::from(String::from("Hello there!"))
        );
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("This"),
                Token::from(","),
                Token::from("is"),
                Token::from("."),
                Token::from("a"),
                Token::from("string"),
                Token::from("."),
                Token::from("That"),
                Token::from("is"),
                Token::from("harder"),
                Token::from("!"),
                Token::from("to"),
                Token::from("parse"),
                Token::from("?"),
                Token::from("Does-"),
                Token::from("it_"),
                Token::from("work"),
                Token::from("?"),
                Token::from("Who"),
                Token::from("Knows"),
                Token::from(";")
            ])),
            Sentence::from(String::from(
                "This, is. a string. That is harder! to parse? Does- it_ work? Who Knows;"
            ))
        );
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("("),
                Token::from("Another"),
                Token::from("difficult"),
                Token::from(":"),
                Token::from("'"),
                Token::from("string"),
                Token::from(")"),
            ])),
            Sentence::from(String::from("(Another difficult: ' string)"))
        );
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("This"),
                Token::from("one"),
                Token::from("has"),
                Token::from("escapes"),
                Token::from(".")
            ])),
            Sentence::from(String::from("This \n one has\r escapes."))
        );
    }

    #[test]
    fn test_from_str_sentence() {
        assert_eq!(Sentence::None, Sentence::from(""));
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("Hello"),
                Token::from("there"),
                Token::from("!")
            ])),
            Sentence::from("Hello there!")
        );
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("This"),
                Token::from(","),
                Token::from("is"),
                Token::from("."),
                Token::from("a"),
                Token::from("string"),
                Token::from("."),
                Token::from("That"),
                Token::from("is"),
                Token::from("harder"),
                Token::from("!"),
                Token::from("to"),
                Token::from("parse"),
                Token::from("?"),
                Token::from("Does-"),
                Token::from("it_"),
                Token::from("work"),
                Token::from("?"),
                Token::from("Who"),
                Token::from("Knows"),
                Token::from(";")
            ])),
            Sentence::from(
                "This, is. a string. That is harder! to parse? Does- it_ work? Who Knows;"
            )
        );
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("("),
                Token::from("Another"),
                Token::from("difficult"),
                Token::from(":"),
                Token::from("'"),
                Token::from("string"),
                Token::from(")"),
            ])),
            Sentence::from("(Another difficult: ' string)")
        );
        assert_eq!(
            Sentence::Text(Box::new([
                Token::from("This"),
                Token::from("one"),
                Token::from("has"),
                Token::from("escapes"),
                Token::from(".")
            ])),
            Sentence::from("This \n one has\r escapes.")
        );
    }

    #[test]
    fn test_add_sentence() {
        assert_eq!(
            Sentence::from(String::from("")) + Sentence::from(String::from("")),
            Sentence::None
        );
        assert_eq!(
            Sentence::from(String::from("Hello")) + Sentence::from(String::from("")),
            Sentence::from(String::from("Hello"))
        );
        assert_eq!(
            Sentence::from(String::from("")) + Sentence::from(String::from("Hello")),
            Sentence::from(String::from("Hello"))
        );
        assert_eq!(
            Sentence::from(String::from("Hello"))
                + Sentence::from(String::from(" "))
                + Sentence::from(String::from("There")),
            Sentence::from(String::from("Hello There"))
        );
    }

    #[test]
    //End of Sentence tests
    #[test]
    fn test_new_ngram() {}
}
