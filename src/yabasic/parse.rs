use std::string::String;

use nom::{
    IResult,
    do_parse,
    named, one_of,
    map,
    take_while,
    re_capture,
};

const IDENTIFIER_START: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
const IDENTIFIER_CHAR: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_0123456789";

fn is_identifier_char(c: char) -> bool {
    IDENTIFIER_CHAR.find(c).is_some()
}

fn combine((first, rest): (char, &str)) -> String {
    let mut s = String::new();
    s.reserve(s.len() + 1);
    s.push(first);
    s.push_str(rest);
    s
}

named!(identifier<&str, &str>,
    map!(
        re_capture!("[a-zA-Z_][a-zA-Z0-9]*"),
        |v| v[0]
    )
);

pub fn parse(s: &str) -> IResult<&str, &str> {
    identifier(s)
}
