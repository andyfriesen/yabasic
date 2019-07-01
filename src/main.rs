extern crate nom;

mod yabasic;

fn main() {
    let res = yabasic::lex::lex("yo");
    println!("Hello, world! {:?}", res);
}
