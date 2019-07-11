
use crate::yabasic::lex;
use crate::yabasic::lex::Token;
use std::slice::Iter;
use std::string::String;
use std::collections::HashMap;

pub struct Program {

}

pub type Offset = usize;
type SrcOffset = usize;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Label {
    Num(Offset),
    Name(String),
}

struct Compiler<'a> {
    bytecode: Vec<u32>,
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

    fn peek(&self) -> Option<Token> {
        // if self.pos < self.tokens.len() {
        //     None
        // } else {
        //     Some(self.tokens[self.pos].clone())
        // }
        None
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
                _ => Ok(())
            }
        } else {
            Err(EOF)
        }
    }
}

pub fn compile(tokens: Vec<Token>) {
    let mut c = Compiler {
        tokens: tokens,
        bytecode: Vec::new(),
        labels: HashMap::new(),
        label_uses: Vec::new(),
        pos: 0,
    };

    loop {
        let done = c.statement();
        if let Err(EOF) = done { break }
    }
}
