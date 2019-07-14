
mod compile_tests {
    use crate::yabasic::lex::*;
    use crate::yabasic::compile::*;

    #[test]
    fn print_blank_line() {
        let (_, tokens) = lex("10 print").unwrap();
        let bytecode = compile(tokens).unwrap();
        assert_eq!(vec!(Op::Print), bytecode.ops);
    }

    #[test]
    fn hello_world() {
        let (_, tokens) = lex("10 print \"Hello, World!\"").unwrap();
        let bytecode = compile(tokens).unwrap();
        assert_eq!(vec!(Op::PushStr(StringIndex(0)), Op::Print), bytecode.ops);
    }
}
