
mod parse_tests {
    use crate::yabasic::lex::*;

    #[test]
    fn identifier_() {
        let (rest, res) = lex("yo").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, [Token::Identifier("yo")]);
    }

    #[test]
    fn keyword() {
        let (rest, res) = lex("print").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, [Token::Keyword(Keyword::Print)]);
    }

    #[test]
    fn integer() {
        let (rest, res) = lex("8675309").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, [Token::Integer(8675309)]);
    }

    #[test]
    fn multiple_tokens() {
        let (rest, res) = lex(" print   hello_world ").unwrap();
        assert_eq!(res, [Token::Keyword(Keyword::Print), Token::Identifier("hello_world")]);
        assert_eq!(rest, "");
    }

    #[test]
    fn comment_() {
        assert!(comment("'a\n").is_ok());
        assert!(comment("'b").is_ok());
        assert!(comment("' A somewhat realistic comment\n").is_ok());
    }

    #[test]
    fn comments_between_tokens() {
        let (rest, res) = lex("print\nfoo\n' a totally useful comment\nprint bar").unwrap();

        assert_eq!(res, [
            Token::Keyword(Keyword::Print),
            Token::Newline,
            Token::Identifier("foo"),
            Token::Newline,
            Token::Newline,
            Token::Keyword(Keyword::Print),
            Token::Identifier("bar")
        ]);
        assert_eq!(rest, "");
    }

    #[test]
    fn space_() {
        assert!(space("").is_ok());
        assert!(space(" ").is_ok());
    }

    #[test]
    fn string_literals() {
        let (rest, res) = lex("print \"Hello World!\"").unwrap();

        assert_eq!(res, [
            Token::Keyword(Keyword::Print),
            Token::String("\"Hello World!\""),
        ]);
        assert_eq!(rest, "");
    }
}
