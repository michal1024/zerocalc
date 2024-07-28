use std::str::Chars;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub len: usize
}

impl Token {
    fn new(kind: TokenKind, start: usize, len: usize) -> Token {
        Token {
            kind,
            start,
            len
        }
    }

    pub fn value_from<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start..self.start + self.len]
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::new(TokenKind::Unknown, 0, 0)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenKind {
    /// 123, 0.123, "abc" etc.
    Literal(LiteralKind),
    /// Identifier
    Ident,
    /// +
    Add,
    /// -
    Sub,
    /// /
    Div,
    /// *
    Mul,
    /// %
    Mod,
    /// ^
    Pow,
    /// (
    Lpar,
    /// )
    Rpar,
    /// ,
    Coma,
    /// =
    Assign,
    /// not recognized
    Unknown,
    /// end of input
    Eof
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LiteralKind {
    Int(Base),
    Float,
    String,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16
}

pub const EOF_CHAR: char = '\0';

fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9' | ' ' | '_')
}

fn is_hex_digit(c: char) -> bool {
    is_digit(c) || matches!(c, 'a'..='f' | 'A'..='F')
}


fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    len_total: usize,
    len_remaining: usize
}

impl Tokenizer<'_> {
    pub fn new(input: &str) -> Tokenizer<'_> {
        Tokenizer {
            chars: input.chars(),
            len_total: input.len(),
            len_remaining: input.len()
        }
    }

    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn reset_token_position(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    fn current_token_start(&mut self) -> usize {
        self.len_total - self.len_remaining
    }

    fn current_token_len(&mut self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    pub fn next_token(&mut self) -> Token {
        self.do_while(is_whitespace);
        let char = self.first();
        let token_kind = match char {
            '0'..='9' | '.' => {
                let number_kind: LiteralKind = self.do_number();
                TokenKind::Literal(number_kind)
            },
            '"' => {
                let literal_kind = self.do_string();
                TokenKind::Literal(literal_kind)
            }
            c if is_ident_start(c) => {
                self.do_while(is_ident);
                TokenKind::Ident
            }
            '+' => { self.bump(); TokenKind::Add }
            '-' => { self.bump(); TokenKind::Sub },
            '/' => { self.bump(); TokenKind::Div },
            '*' => { self.bump(); TokenKind::Mul },
            '^' => { self.bump(); TokenKind::Pow },
            '%' => { self.bump(); TokenKind::Mod },
            '(' => { self.bump(); TokenKind::Lpar },
            ')' => { self.bump(); TokenKind::Rpar },
            ',' => { self.bump(); TokenKind::Coma },
            '=' => { self.bump(); TokenKind::Assign },
            EOF_CHAR => TokenKind::Eof,
            _ => TokenKind::Unknown
        };
        let token = Token::new(token_kind, self.current_token_start(), self.current_token_len());
        self.reset_token_position();
        
        token
    }

    fn do_number(&mut self) -> LiteralKind {
        let mut kind = LiteralKind::Int(Base::Dec);
        if self.first() == '0' {
            self.bump();
            match self.first() {
                'b' => {
                    self.bump();
                    self.do_while(is_digit);
                    return LiteralKind::Int(Base::Bin)
                },
                'o' => {
                    self.bump();
                    self.do_while(is_digit);
                    return LiteralKind::Int(Base::Oct)
                },
                'x' => {
                    self.bump();
                    self.do_while(is_hex_digit);
                    return LiteralKind::Int(Base::Hex)
                },
                _ => ()
            }
        }
        self.do_while(is_digit);

        if self.first() == '.' {
            kind = LiteralKind::Float;
            self.bump();
            self.do_while(is_digit);
        }
        if matches!(self.first(), 'e' | 'E') {
            kind = LiteralKind::Float;
            self.bump();
            self.do_exp();
        }
    kind
    }

    fn do_exp(&mut self) {
        self.do_while(is_whitespace);
        if matches!(self.first(), '-' | '+') {
            self.bump();
        }
        self.do_while(is_digit);
    }

    fn do_string(&mut self) -> LiteralKind {
        self.bump(); // advance past opening '"'
        loop {
            match self.first() {
                '"' => {
                    // consume closing '"' and stop
                    self.bump();
                    break
                },
                '\\' => {
                    // skip over whatever is escaped
                    self.bump();
                },
                _ => ()
            }
            self.bump();
        }
        LiteralKind::String
    }

    fn do_while<P>(&mut self, predicate: P)
        where P : Fn(char) -> bool
    {
        while predicate(self.first()) {
            self.bump();
        }
    }
}

