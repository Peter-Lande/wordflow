pub mod tokenization;

#[cfg(test)]
mod tests {
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
}
