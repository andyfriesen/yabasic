
use std::vec::Vec;

use nom::{
    IResult,
    alt,
    many0,
    map,
    named,
    re_capture,
    ws,
};

pub type Int = i32;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    String(&'a str),
    Keyword(Keyword),
    Integer(Int),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Print,
}

use Token::{String, Integer};
use Keyword::*;

named!(identifier<&str, Token>,
    map!(
        re_capture!("[a-zA-Z_][a-zA-Z0-9_]*"),
        |v| match v[0] {
            "print" => Token::Keyword(Print),
            a => String(a)
        }
    )
);

named!(integer<&str, Token>,
    map!(
        re_capture!("[0-9]+"),
        |v| Integer(v[0].parse().unwrap())
    )
);

named!(token<&str, Token>,
    alt!(identifier | integer)
);

named!(tokens<&str, Vec<Token>>,
    many0!(ws!(token))
);

pub fn lex(s: &str) -> IResult<&str, Vec<Token>> {
    tokens(s)
}
