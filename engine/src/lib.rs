mod lexer;
pub mod eval;
pub mod parser;
pub mod errors;
pub mod number;
pub mod function;

pub use errors::Error;
pub use parser::Parser;
pub use eval::Calculator;