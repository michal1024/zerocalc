use super::*;

#[test]
fn test_function() {
    let f = parse_function("sin").unwrap();
    let mut args = vec![Number::Float(std::f64::consts::PI)];
    match f.exec(&mut args) {
        Number::Float(f) => assert!(f.abs() < 1e-10),
        _ => panic!("Expected float")
    }
}

#[test]
fn test_function2() {
    let f = parse_function("log").unwrap();
    let mut args = vec![
        Number::Float(8.0), 
        Number::Float(2.0)];
    match f.exec(&mut args) {
        Number::Float(f) => assert!((f-3.0).abs() < 1e-10),
        _ => panic!("Expected float")
    }
}
