pub mod tokenization;

#[cfg(test)]
mod tests {
    use crate::tokenization::Token;

    #[test]
    fn test_from_string() {
        assert_eq!(Token::None, Token::from(String::from("")));
        assert_eq!(
            Token::Text(Box::new([
                String::from("Hello"),
                String::from("there"),
                String::from("!")
            ])),
            Token::from(String::from("Hello there!"))
        );
        assert_eq!(
            Token::Text(Box::new([
                String::from("This"),
                String::from(","),
                String::from("is"),
                String::from("."),
                String::from("a"),
                String::from("string"),
                String::from("."),
                String::from("That"),
                String::from("is"),
                String::from("harder"),
                String::from("!"),
                String::from("to"),
                String::from("parse"),
                String::from("?"),
                String::from("Does"),
                String::from("-"),
                String::from("it"),
                String::from("_"),
                String::from("work"),
                String::from("?"),
                String::from("Who"),
                String::from("Knows"),
                String::from(";")
            ])),
            Token::from(String::from(
                "This, is. a string. That is harder! to parse? Does- it_ work? Who Knows;"
            ))
        );
        assert_eq!(
            Token::Text(Box::new([
                String::from("("),
                String::from("Another"),
                String::from("difficult"),
                String::from(":"),
                String::from("'"),
                String::from("string"),
                String::from(")"),
            ])),
            Token::from(String::from("(Another difficult: ' string)"))
        );
        assert_eq!(
            Token::Text(Box::new([
                String::from("This"),
                String::from("one"),
                String::from("has"),
                String::from("escapes"),
                String::from(".")
            ])),
            Token::from(String::from("This \n one has\r escapes."))
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Token::None, Token::from(""));
        assert_eq!(
            Token::Text(Box::new([
                String::from("Hello"),
                String::from("there"),
                String::from("!")
            ])),
            Token::from("Hello there!")
        );
        assert_eq!(
            Token::Text(Box::new([
                String::from("This"),
                String::from(","),
                String::from("is"),
                String::from("."),
                String::from("a"),
                String::from("string"),
                String::from("."),
                String::from("That"),
                String::from("is"),
                String::from("harder"),
                String::from("!"),
                String::from("to"),
                String::from("parse"),
                String::from("?"),
                String::from("Does"),
                String::from("-"),
                String::from("it"),
                String::from("_"),
                String::from("work"),
                String::from("?"),
                String::from("Who"),
                String::from("Knows"),
                String::from(";")
            ])),
            Token::from("This, is. a string. That is harder! to parse? Does- it_ work? Who Knows;")
        );
        assert_eq!(
            Token::Text(Box::new([
                String::from("("),
                String::from("Another"),
                String::from("difficult"),
                String::from(":"),
                String::from("'"),
                String::from("string"),
                String::from(")"),
            ])),
            Token::from("(Another difficult: ' string)")
        );
        assert_eq!(
            Token::Text(Box::new([
                String::from("This"),
                String::from("one"),
                String::from("has"),
                String::from("escapes"),
                String::from(".")
            ])),
            Token::from("This \n one has\r escapes.")
        );
    }

    #[test]
    fn test_add_token() {
        assert_eq!(
            Token::from(String::from("")) + Token::from(String::from("")),
            Token::None
        );
        assert_eq!(
            Token::from(String::from("Hello")) + Token::from(String::from("")),
            Token::from(String::from("Hello"))
        );
        assert_eq!(
            Token::from(String::from("")) + Token::from(String::from("Hello")),
            Token::from(String::from("Hello"))
        );
        assert_eq!(
            Token::from(String::from("Hello"))
                + Token::from(String::from(" "))
                + Token::from(String::from("There")),
            Token::from(String::from("Hello There"))
        );
    }
}
