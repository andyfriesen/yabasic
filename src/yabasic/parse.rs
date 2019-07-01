
use nom::{
    IResult,
    named,
    map,
    alt,
    re_capture,
};

pub type Int = i32;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    String(&'a str),
    Integer(Int),
}

named!(identifier<&str, Token>,
    map!(
        re_capture!("[a-zA-Z_][a-zA-Z0-9]*"),
        |v| Token::String(v[0])
    )
);

named!(integer<&str, Token>,
    map!(
        re_capture!("[0-9]+"),
        |v| Token::Integer(v[0].parse().unwrap())
    )
);

named!(expr<&str, Token>,
    alt!(identifier | integer)
);

pub fn parse(s: &str) -> IResult<&str, Token> {
    expr(s)
}
