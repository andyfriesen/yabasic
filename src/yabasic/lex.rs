
use std::vec::Vec;

use nom::{
    IResult,
    alt,
    do_parse,
    many0,
    map,
    named,
    opt,
    re_capture,
};

pub type Int = i64;

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    String(&'a str),
    Identifier(&'a str),
    Keyword(Keyword),
    Integer(Int),
    Newline,
    Colon,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Print,
}

use Token::{Identifier, Integer};
use Keyword::*;

named!(pub space<&str, Vec<&str>>,
    re_capture!("^[\t ]*")
);

named!(newline<&str, Token>,
    do_parse!(
        re_capture!("^[\n\r]") >>
        (Token::Newline)
    )
);

named!(colon<&str, Token>,
    do_parse!(
        re_capture!("^:") >> (Token::Colon)
    )
);

named!(identifier<&str, Token>,
    map!(
        re_capture!("^[a-zA-Z_][a-zA-Z0-9_]*"),
        |v| match v[0] {
            "print" => Token::Keyword(Print),
            a => Identifier(a)
        }
    )
);

named!(string<&str, Token>,
    map!(
        re_capture!("^\"[^\"]*\""),
        |v| Token::String(v[0])
    )
);

named!(integer<&str, Token>,
    map!(
        re_capture!("^[0-9]+"),
        |v| Integer(v[0].parse().unwrap())
    )
);

named!(pub comment<&str, Vec<&str>>,
    re_capture!("^'[^\n]*")
);

named!(token<&str, Token>,
    do_parse!(
        space >>
        p: alt!(identifier | integer | string | colon | newline) >>
        space >>
        opt!(comment) >>
        (p)
    )
);

named!(tokens<&str, Vec<Token>>,
    many0!(token)
);

pub fn lex(s: &str) -> IResult<&str, Vec<Token>> {
    tokens(s)
}
