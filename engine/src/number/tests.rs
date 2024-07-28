use super::*;
use super::parse::*;

#[test]
fn test_parse_int_whitespaces() {
    let i = parse_int("1 2_30 1").unwrap();
    assert_eq!(Number::Int(12301), i);
}


#[test]
fn test_parse_bin() {
    let i = parse_int_bin("0b01101").unwrap();
    assert_eq!(Number::Int(13), i);
    let i = parse_int_bin("0b_01110").unwrap();
    assert_eq!(Number::Int(14), i);
}


#[test]
fn test_parse_oct() {
    let i = parse_int_oct("0o10").unwrap();
    assert_eq!(Number::Int(8), i);
    let r = parse_int_oct("0o8");
    assert!(r.is_err());
}

#[test]
fn test_parse_hex() {
    let i = parse_int_hex("0xaB_c1").unwrap();
    assert_eq!(Number::Int(43969), i);
}

#[test]
fn test_parse_float() {
    let n = parse_float("1.23").unwrap();
    if let Number::Float(f) = n {
        assert!((1.23-f).abs() < 1e-10);
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_float_exp() {
    let n = parse_float(".23e-1").unwrap();
    match n {
        Number::Float(f) => assert!((0.23e-1-f).abs() < 1e-10),
        _ => panic!("Not a float")
    }
}


#[test]
fn test_div_zero_float() {
    let (l, r) = (Number::Float(1.0), Number::Float(0.0));
    let res = l / r;
    assert_eq!(res, Number::NaN);
}

#[test]
fn test_div_zero_int() {
    let (l, r) = (Number::Int(1), Number::Int(0));
    let res = l / r;
    assert_eq!(res, Number::NaN);
}

#[test]
fn test_rem() {
    let (l, r) = (Number::Int(3), Number::Int(0));
    let res = l % r;
    assert_eq!(res, Number::NaN);

    let (l, r) = (Number::Int(3), Number::Int(2));
    let res = l % r;
    assert_eq!(res, Number::Int(1));
}