
mod parse_tests {
    use crate::yabasic::parse::*;

    #[test]
    fn identifier() {
        let (rest, res) = parse("yo").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Token::String("yo"));
    }

    #[test]
    fn keyword() {
        let (rest, res) = parse("print").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Token::Keyword(Keyword::Print));
    }

    #[test]
    fn integer() {
        let (rest, res) = parse("8675309").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Token::Integer(8675309));
    }
}
