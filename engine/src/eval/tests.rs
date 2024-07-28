use super::*;
use crate::function;

fn eval(prog: & [Expression]) -> Number {
    let mut c = Calculator::new();
    c.eval(prog)
}

#[test]
fn test_val() {
    let program = [Expression::Val(1.into())];
    assert_eq!(eval(&program), 1.into());
}

#[test]
fn test_binary_op_int() {
    let program = [
        Expression::Val(2.into()),
        Expression::Val(3.into()),
        Expression::BinaryOp(Op::Add)
    ];
    assert_eq!(eval(&program), 5.into());
}

#[test]
fn test_div_ints() {
    let program = [
        Expression::Val(4.into()),
        Expression::Val(2.into()),
        Expression::BinaryOp(Op::Div)
    ];
    assert_eq!(eval(&program), 2.into());

    let program = [
        Expression::Val(2.into()),
        Expression::Val(3.into()),
        Expression::BinaryOp(Op::Div)
    ];
    if let Number::Float(f) = eval(&program) {
        assert!((f - 2.0/3.0).abs() < 0.01)
    } else {
        assert!(false)
    }
}

#[test]
fn test_unary_minus() {
    let program = [
        Expression::Val(2.into()),
        Expression::UnaryOp(Op::Sub)
    ];
    assert_eq!(Number::Int(-2), eval(&program));
}

#[test]
fn test_function_eval() {
    let program = vec![
        Expression::Val(1.into()),
        Expression::Val(1.into()),
        Expression::BinaryOp(Op::Sub),
        Expression::FuncCall(function::parse_function("sin").unwrap())
    ];
    match eval(&program) {
        Number::Float(f) => assert!(f.abs() < 1e-10),
        _ => panic!("expected 0.0")
    }
}

#[test]
fn test_assign() {
    let p1 = vec![
        Expression::Val(1.into()),
        Expression::Assign(Ident::new("x"))
    ];
    let p2 = vec![
        Expression::Val(2.into()),
        Expression::Ref(Ident::new("x")),
        Expression::BinaryOp(Op::Add)
    ];
    let mut c = Calculator::new();
    c.eval(&p1);
    let res = c.eval(&p2);
    assert_eq!(Number::Int(3), res);
}