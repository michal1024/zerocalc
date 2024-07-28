use crate::number::Number;
use crate::errors::Error;

#[cfg(test)]
mod tests;

pub type FnPtr = fn(&mut Vec<Number>) -> Number;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub ptr: FnPtr,
    pub arg_count: usize
}

impl Function {
    fn new(ptr: FnPtr, arg_count: usize) -> Function {
        Function {
            ptr,
            arg_count
        }
    }

    pub fn exec(&self, args: &mut Vec<Number>) -> Number {
        (self.ptr)(args)
    }
}


macro_rules! f64_function {
    ($n:ident) => {
        fn $n(stack: &mut Vec<Number>) -> Number {
            let arg: f64 = stack.pop().unwrap_or_default().into();
            arg.$n().into()
        }        
    }
}

f64_function!(sin);
f64_function!(cos);
f64_function!(tan);
f64_function!(asin);
f64_function!(acos);
f64_function!(atan);
f64_function!(ln);
f64_function!(log10);
f64_function!(sqrt);

fn abs(stack: &mut Vec<Number>) -> Number {
    let n = stack.pop().unwrap_or_default();
    match n {
        Number::NaN => Number::NaN,
        Number::Float(f) => f.abs().into(),
        Number::Int(i) => i.abs().into()
    }
}

fn log(stack: &mut Vec<Number>) -> Number {
    let rhs = stack.pop().unwrap_or_default();
    let lhs = stack.pop().unwrap_or_default();
    if rhs == Number::NaN || lhs == Number::NaN {
        return Number::NaN
    }
    let l: f64 = lhs.into();
    let r: f64 = rhs.into();
    l.log(r).into()
}

fn root(stack: &mut Vec<Number>) -> Number {
    let rhs = stack.pop().unwrap_or_default();
    let lhs = stack.pop().unwrap_or_default();
    if rhs == Number::NaN || lhs == Number::NaN {
        return Number::NaN
    }
    let l: f64 = lhs.into();
    let r: f64 = rhs.into();
    l.powf(1.0/r).into()
}

pub fn parse_function(input: &str) -> Result<Function, Error> {
    let name = input.trim();
    let f = match name {
        "sin" => Function::new(sin, 1),
        "cos" => Function::new(cos, 1),
        "tan" => Function::new(tan, 1),
        "asin" => Function::new(asin, 1),
        "acos" => Function::new(acos, 1),
        "atan" => Function::new(atan, 1),
        "ln" => Function::new(ln, 1),
        "log10" => Function::new(log10, 1),
        "sqrt" => Function::new(sqrt, 1),
        "root" => Function::new(root, 2),
        "abs" => Function::new(abs, 1),
        "log" => Function::new(log, 2),
        _ => return Err(format!("Unknown function {name}").into())
    };
    Ok(f)
}
