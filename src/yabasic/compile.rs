
use crate::yabasic::lex::Token;
use crate::yabasic::lex::Keyword;
use std::string::String;
use std::collections::HashMap;

pub type Offset = usize;
type SrcOffset = usize;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Label {
    Num(Offset),
    Name(String),
}

#[derive(Debug)]
pub struct Bytecode {
    pub ops: Vec<Op>,
}

impl Bytecode {
    fn len(&self) -> usize {
        self.ops.len()
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Print = 0x01,
}

pub struct Compiler<'a> {
    bytecode: Bytecode,
    labels: HashMap<Label, Offset>,
    label_uses: Vec<(u32, Label)>, // Positions where labels are referred to.  We'll fill in the real offsets in a second pass.

    tokens: Vec<Token<'a>>,
    pos: SrcOffset, // current offset into the token stream
}

enum CompileError {
    EOF,
}

use CompileError::*;

impl<'a> Compiler<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Compiler<'a> {
        Compiler {
            tokens: tokens,
            bytecode: Bytecode { ops: Vec::new() },
            labels: HashMap::new(),
            label_uses: Vec::new(),
            pos: 0,
        }
    }

    fn pos(&self) -> Offset {
        self.bytecode.len()
    }

    fn eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn next(&mut self) -> Option<&Token> {
        if self.eof() {
            None
        } else {
            let res = Some(&self.tokens[self.pos]);
            self.pos += 1;
            res
        }
    }

    fn peek(&self) -> Option<Token<'a>> {
        if self.eof() {
            None
        } else {
            Some(self.tokens[self.pos].clone())
        }
    }

    fn emit(&mut self, op: Op) {
        self.bytecode.ops.push(op);
    }

    fn statement(&mut self) -> Result<(), CompileError> {
        let tok = if let Some(t) = self.peek() {
            t.clone()
        } else {
            return Err(EOF)
        };

        self.pos += 1;

        match tok {
            Token::Integer(line_num) if line_num >= 0 => {
                let ln = line_num as usize;
                let pos = self.pos();
                self.labels.insert(Label::Num(ln), pos);
                self.statement_no_line_num()
            },
            _ =>
                self.statement_no_line_num()
        }
    }

    fn statement_no_line_num(&mut self) -> Result<(), CompileError> {
        if let Some(tok) = self.next() {
            match tok {
                Token::Keyword(Keyword::Print) => {
                    self.emit(Op::Print);
                    Ok(())
                },
                _ => Ok(())
            }
        } else {
            Err(EOF)
        }
    }
}

pub fn compile(tokens: Vec<Token>) -> Bytecode {
    let mut c = Compiler::new(tokens);
    loop {
        let done = c.statement();
        if let Err(EOF) = done { break }
    }

    c.bytecode
}
