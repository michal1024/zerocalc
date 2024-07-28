use super::Number;
use crate::errors::Error;

fn filter_whitespace(c: &char) -> bool {
    !c.is_whitespace() && *c != '_'
}

fn sanitize(input: &str) -> String {
    input.chars().filter(filter_whitespace).collect()
}

pub fn parse_int(input: &str) -> Result<Number, Error> {
    let s = sanitize(input);
    let val = s.parse().map_err(Error::wrap)?;
    Ok(Number::Int(val))
}

pub fn parse_int_bin(input: &str) -> Result<Number, Error> {
    let s = sanitize(input);
    if !s.starts_with("0b") {
        return Err("Binary number must start with 0b".into());
    }
    let val = i128::from_str_radix(&s[2..], 2).map_err(Error::wrap)?;
    Ok(Number::Int(val))
}

pub fn parse_int_oct(input: &str) -> Result<Number, Error> {
    let s = sanitize(input);
    if !s.starts_with("0o") {
        return Err("Octal number must start with 0o".into());
    }
    let val =i128::from_str_radix(&s[2..], 8).map_err(Error::wrap)?;
    Ok(Number::Int(val))
}

pub fn parse_int_hex(input: &str) -> Result<Number, Error> {
    let s = sanitize(input);
    if !s.starts_with("0x") {
        return Err("Hex number must start with 0x".into());
    }
    let val = i128::from_str_radix(&s[2..], 16).map_err(Error::wrap)?;
    Ok(Number::Int(val))
}

pub fn parse_float(input: &str) -> Result<Number, Error> {
    let s = sanitize(input);
    let val = s.parse().map_err(Error::wrap)?;
    Ok(Number::Float(val))
}

pub fn parse_const(input: &str) -> Result<Number, Error> {
    match input {
        "pi" => Ok(Number::Float(std::f64::consts::PI)),
        "e" => Ok(Number::Float(std::f64::consts::E)),
        _ => Err("Unknown constant".into())
    }
}
