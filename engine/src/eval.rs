use crate::number::Number;
use crate::function::Function;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Add, // a + b
    Sub, // a - b
    Mul, // a * b
    Div, // a / b
    Mod, // a % b
    Pow  // a ^ b
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new(s: &str) -> Self {
        Ident(String::from(s))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Val(Number),
    BinaryOp(Op),
    UnaryOp(Op),
    FuncCall(Function),
    Assign(Ident),
    Ref(Ident)
}

pub struct Calculator {
    stack: Vec<Number>,
    vars: HashMap<Ident, Number>
}

impl Calculator {

    pub fn new() -> Self {
        Calculator {
            stack: vec![],
            vars: HashMap::new()
        }
    }

    pub fn eval(&mut self, program: &[Expression]) -> Number {
        self.stack.clear();
        for ex in program {
            match ex {
                Expression::Val(n) => self.stack.push(*n),
                Expression::BinaryOp(op) => self.eval_binary_op(*op),
                Expression::UnaryOp(op) => self.eval_unary_op(*op),
                Expression::FuncCall(f) => self.eval_func_call(f),
                Expression::Assign(id) => self.eval_assign(id),
                Expression::Ref(id) => self.eval_ref(id)
            }
        }
        self.stack.pop().unwrap_or_default()
    }

    fn eval_binary_op(&mut self, op: Op) {
        let r = self.stack.pop().unwrap_or_default();
        let l = self.stack.pop().unwrap_or_default();
        let res = match op {
            Op::Add => l + r,
            Op::Sub => l - r,
            Op::Mul => l * r,
            Op::Div => l / r,
            Op::Mod => l % r,
            Op::Pow => l ^ r
        };
        self.stack.push(res);
    }

    fn eval_unary_op(&mut self, op: Op) {
        let arg = self.stack.pop().unwrap_or_default();
        let res = match op {
            Op::Add => arg,
            Op::Sub => -arg,
            _ => Number::NaN
        };
        self.stack.push(res);
    }

    fn eval_func_call(&mut self, f: &Function) {
        let res = f.exec(&mut self.stack);
        self.stack.push(res);
    }

    fn eval_assign(&mut self, id: &Ident) {
        let val = self.stack.pop().unwrap_or_default();
        self.vars.insert(id.clone(), val);
        self.stack.push(val);
    }

    fn eval_ref(&mut self, id: &Ident) {
        let val = match self.vars.get(id) {
            Some(n) => n.clone(),
            None => Number::NaN
        };
        self.stack.push(val);
    }
}

