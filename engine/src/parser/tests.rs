use super::*;
use crate::eval::Expression;
use crate::number::Number;

#[test]
fn test_single_val() {
    let mut p = Parser::new("1");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1))
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_add() {
    let mut p = Parser::new("1+2");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1)),
        Expression::Val(Number::Int(2)),
        Expression::BinaryOp(Op::Add)
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_precedence() {
    let mut p = Parser::new("1*0b10*3+4");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1)),
        Expression::Val(Number::Int(2)),
        Expression::Val(Number::Int(3)),
        Expression::BinaryOp(Op::Mul),
        Expression::BinaryOp(Op::Mul),
        Expression::Val(Number::Int(4)),
        Expression::BinaryOp(Op::Add)
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_precedence2() {
    let mut p = Parser::new("1+2*3*4");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1)),
        Expression::Val(Number::Int(2)),
        Expression::Val(Number::Int(3)),
        Expression::Val(Number::Int(4)),
        Expression::BinaryOp(Op::Mul),
        Expression::BinaryOp(Op::Mul),
        Expression::BinaryOp(Op::Add)
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_precedence3() {
    let mut p = Parser::new("1*2+3*4");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1)),
        Expression::Val(Number::Int(2)),
        Expression::BinaryOp(Op::Mul),
        Expression::Val(Number::Int(3)),
        Expression::Val(Number::Int(4)),
        Expression::BinaryOp(Op::Mul),
        Expression::BinaryOp(Op::Add)
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_unary() {
    let mut p = Parser::new("-1");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1)),
        Expression::UnaryOp(Op::Sub)
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_paren_and_whitespaces() {
    let mut p = Parser::new(" 2 + (  -  1 + 3 ) ");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(2)),
        Expression::Val(Number::Int(1)),
        Expression::UnaryOp(Op::Sub),
        Expression::Val(Number::Int(3)),
        Expression::BinaryOp(Op::Add),
        Expression::BinaryOp(Op::Add)
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_const() {
    let mut p = Parser::new("pi");
    p.parse().unwrap();
    assert_eq!(p.program.len(), 1);
    match p.program[0] {
        Expression::Val(Number::Float(f)) => {
            assert!((f - 3.14).abs() < 0.01)
        },
        _ => panic!("Wrong expression")
    }
}

#[test]
fn test_functions() {
    let mut p = Parser::new("sin(1-1)");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(Number::Int(1)),
        Expression::Val(Number::Int(1)),
        Expression::BinaryOp(Op::Sub),
        Expression::FuncCall(function::parse_function("sin").unwrap())
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_assign() {
    let mut p = Parser::new("x = 1 + 2");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(1.into()),
        Expression::Val(2.into()),
        Expression::BinaryOp(Op::Add),
        Expression::Assign(Ident::new("x"))
    ];
    assert_eq!(expected, p.program);
}

#[test]
fn test_ident() {
    let mut p = Parser::new("1 + x");
    p.parse().unwrap();
    let expected = vec![
        Expression::Val(1.into()),
        Expression::Ref(Ident::new("x")),
        Expression::BinaryOp(Op::Add),
    ];
    assert_eq!(expected, p.program);
}