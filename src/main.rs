extern crate nom;

mod yabasic;

fn main() {
    let res = yabasic::lex::lex("yo");
    match res {
        Ok((_, tokens)) => {
            let bytecode = yabasic::compile::compile(tokens);
            println!("ok! {:?}", bytecode);
        },
        Err(e) => {
            println!("Lexer error! {:?}", e);
        }
    }
}
