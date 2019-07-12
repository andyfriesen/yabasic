
mod compile_tests {
    use crate::yabasic::lex::*;
    use crate::yabasic::compile::*;

    #[test]
    fn hello() {
        let (_, tokens) = lex("10 print").unwrap();
        let bytecode = compile(tokens);
        assert_eq!(vec!(Op::Print), bytecode.ops);
    }
}
