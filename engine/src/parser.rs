#[cfg(test)]
mod tests;

use crate::eval::{Expression, Op, Ident};
use crate::{function, lexer};
use crate::number::Number;
use crate::number::parse;
use crate::errors::{Error, Span};
use std::mem;

pub struct Parser<'src> {
    pub program: Vec<Expression>,
    source: &'src str,
    tokens: lexer::Tokenizer<'src>,
    current_token: lexer::Token,
    next_token: lexer::Token
}

static ERR_UNEXP: &str = "Unexpected token";
static ERR_EOF: &str = "Unexpected end of input";

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Parser<'src> {
        Parser {
            source,
            tokens: lexer::Tokenizer::new(source),
            program: vec![],
            current_token: Default::default(),
            next_token: Default::default(),
        }
    }

    fn bump(&mut self) {
        self.current_token = mem::replace(
            &mut self.next_token, 
            self.tokens.next_token());
    }

    fn init(&mut self) {
        self.current_token = self.tokens.next_token();
        self.next_token = self.tokens.next_token();
    }

    fn current_token_value(&self) -> &'src str {
        &self.current_token.value_from(&self.source)
    }

    fn current_span(&self) -> Span {
        Span::new(self.current_token.start, self.current_token.len)
    }

    fn error(&self, message: &str) -> Result<bool, Error> {
        Err(Error::new(message, self.current_span()))
    }

    // exp: assign | exp1 | empty
    // assign: id = exp1
    // exp1: exp2 op1 exp1 | exp2
    // op1: + | -
    // exp2: exp3 op2 exp2 | exp3
    // op2: * | / | %
    // exp3: fact op3 exp3 | fact
    // op3: ^
    // fact: +fact | -fact | (exp1) | func | id | literal
    // func: id(exp1)

    pub fn parse(&mut self) -> Result<bool, Error> {
        self.init();
        match self.parse_exp() {
            Err(e) => {
                Err(Error::new(&e.message, self.current_span()))
            },
            Ok(b) => Ok(b)
        }
    }


    // exp: assign | exp1 | empty
    pub fn parse_exp(&mut self) -> Result<bool, Error> {
        match self.current_token.kind {
            lexer::TokenKind::Eof => Ok(false),
            lexer::TokenKind::Ident if self.next_token.kind == lexer::TokenKind::Assign => {
                self.parse_assign()
            },
            _ => self.parse_exp1()
        }
    }

    // exp1: exp2 op1 exp1 | exp2
    // op1: + | -
    fn parse_exp1(&mut self) -> Result<bool, Error> {
        let has = self.parse_exp2()?;
        match self.current_token.kind {
            kind @(lexer::TokenKind::Add | lexer::TokenKind::Sub) => {
                self.bump();
                if self.parse_exp1()? {
                    self.parse_binary_op(kind)?;
                    Ok(true)
                } else {
                    self.error(ERR_EOF)
                }
            },
            _ => Ok(has),
        }
    }

    // id = exp1
    fn parse_assign(&mut self) -> Result<bool, Error> {
        let ident = self.current_token_value().trim();
        self.bump();
        self.bump(); // skip =
        if self.parse_exp1()? {
            self.program.push(Expression::Assign(Ident::new(ident)));
            Ok(true)
        } else {
            self.error("Missing right side of assignment")
        }
    }

    // exp2: exp3 op2 exp2| exp3
    // op2: * | / | %
    fn parse_exp2(&mut self) -> Result<bool, Error> {
        let has = self.parse_exp3()?;
        match self.current_token.kind {
            kind @(lexer::TokenKind::Mul | lexer::TokenKind::Div | lexer::TokenKind::Mod) => {
                self.bump();
                if self.parse_exp2()? {
                    self.parse_binary_op(kind)?;
                    Ok(true)
                } else {
                    self.error(ERR_EOF)
                }
            },
            _ => Ok(has)
        }
    }

    // exp3: fact op3 exp3 | fact
    // op3: ^
    fn parse_exp3(&mut self) -> Result<bool, Error> {
        let has = self.parse_fact()?;
        match self.current_token.kind {
            kind @lexer::TokenKind::Pow => {
                self.bump();
                if self.parse_exp3()? {
                    self.parse_binary_op(kind)?;
                    Ok(true)
                } else {
                    self.error(ERR_EOF)
                }
            },
            _ => Ok(has)
        }
    }

    // fact: +fact | -fact | (exp1) | func | id | literal
    // func: id(exp1)
    fn parse_fact(&mut self) -> Result<bool, Error> {
        match self.current_token.kind {
            kind@ (lexer::TokenKind::Add |lexer::TokenKind::Sub) => {
                self.bump();
                if !self.parse_fact()? {
                    return self.error("Unary operator needs expression");
                }
                self.parse_unary_op(kind)
            }
            lexer::TokenKind::Lpar => {
                self.bump();
                let has = self.parse_exp1()?;
                if self.current_token.kind != lexer::TokenKind::Rpar {
                    return self.error("Missing closing parenthesis");
                };
                self.bump();
                Ok(has)
            },
            lexer::TokenKind::Literal(kind) => {
                self.parse_literal(kind)
            },
            lexer::TokenKind::Ident => {
                match self.next_token.kind {
                    lexer::TokenKind::Lpar => self.parse_function(),
                    lexer::TokenKind::Assign => self.parse_assign(),
                    _ => self.parse_ident()
                }
            }
            lexer::TokenKind::Eof => Ok(false),
            _ => self.error(ERR_UNEXP)
        }
    }

    fn parse_function(&mut self) -> Result<bool, Error> {
        let f = function::parse_function(self.current_token_value())?;
        self.bump();
        self.bump(); // skip "("
        for i in 0..f.arg_count {
            if !self.parse_exp1()? {
                return self.error(&format!("Argument {} is empty", i+1));
            }
            match self.current_token.kind {
                lexer::TokenKind::Coma => self.bump(),
                lexer::TokenKind::Rpar => break,
                _ => {
                    if i == f.arg_count -1 {
                        return self.error("Expected closing bracket");
                    }
                    return self.error("Invalid number of arguments")
                }
            }
        }
        self.program.push(Expression::FuncCall(f));
        self.bump();
        Ok(true)
    }

    fn parse_ident(&mut self) -> Result<bool, Error> {
        let val = self.current_token_value().trim();
        let c = parse::parse_const(val);
        if c.is_ok() {
            self.program.push(Expression::Val(c.unwrap())); 
        } else {
            self.program.push(Expression::Ref(Ident::new(val)))
        }
        self.bump();
        Ok(true)
    }

    fn parse_literal(&mut self, l: lexer::LiteralKind) -> Result<bool, Error> {
        let val = match l {
            lexer::LiteralKind::Int(b) => self.parse_int(b)?,
            lexer::LiteralKind::Float => self.parse_float()?,
            _ => return Err(Error::new("Unknown Literal", self.current_span()))
        };
        self.program.push(Expression::Val(val));
        self.bump();
        Ok(true)
    }

    fn parse_int(&mut self, b: lexer::Base) -> Result<Number, Error> {
        let n = match b {
            lexer::Base::Bin => {
                parse::parse_int_bin(self.current_token_value())?
            },
            lexer::Base::Oct => {
                parse::parse_int_oct(self.current_token_value())?
            },
            lexer::Base::Dec => {
                parse::parse_int(self.current_token_value())?
            },
            lexer::Base::Hex => {
                parse::parse_int_hex(self.current_token_value())?
            }
        };
        Ok(n)
    }

    fn parse_float(&mut self) -> Result<Number, Error> {
        let f = parse::parse_float(&self.current_token_value())?;
        Ok(f)
    }

    fn parse_binary_op(&mut self, kind: lexer::TokenKind) -> Result<bool, Error>{
        let op = match kind {
            lexer::TokenKind::Add => Op::Add,
            lexer::TokenKind::Sub => Op::Sub,
            lexer::TokenKind::Div => Op::Div,
            lexer::TokenKind::Mul => Op::Mul,
            lexer::TokenKind::Mod => Op::Mod,
            lexer::TokenKind::Pow => Op::Pow,
            _ => return self.error("Invalid binary operator")
        };
        self.program.push(Expression::BinaryOp(op));
        Ok(true)
    }

    fn parse_unary_op(&mut self, kind: lexer::TokenKind) -> Result<bool, Error> {
        let op = match kind {
            lexer::TokenKind::Add => Op::Add,
            lexer::TokenKind::Sub => Op::Sub,
            _ => return self.error("Invalid unary operator")
        };
        self.program.push(Expression::UnaryOp(op));
        Ok(true)
    }
}