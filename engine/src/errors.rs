use std::fmt::Display;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Span {
    pub pos: usize,
    pub len: usize
}

impl Span {
    pub fn new(pos: usize, len: usize) -> Self {
        Span {
            pos,
            len
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub span: Span
}

impl Error {
    pub fn wrap<T: Display>(t: T) -> Self {
        Error {
            message: format!("{t}"),
            span: Span::new(0, 0)
        }
    }

    pub fn new(message: &str, span: Span) -> Self {
        Error {
            message: String::from(message),
            span
        }
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error {
            message: String::from(s),
            span: Span::new(0, 0)
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        s.as_str().into()
    }
}