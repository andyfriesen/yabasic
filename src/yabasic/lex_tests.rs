
mod parse_tests {
    use crate::yabasic::lex::*;

    #[test]
    fn identifier() {
        let (rest, res) = lex("yo").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Token::String("yo"));
    }

    #[test]
    fn keyword() {
        let (rest, res) = lex("print").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Token::Keyword(Keyword::Print));
    }

    #[test]
    fn integer() {
        let (rest, res) = lex("8675309").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Token::Integer(8675309));
    }
}
