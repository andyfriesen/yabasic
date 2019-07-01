extern crate nom;

mod yabasic;

fn main() {
    let res = yabasic::parse::parse("yo");
    println!("Hello, world! {:?}", res);
}
