#[cfg(test)]
mod tests;
pub mod parse;

use std::ops::{Add, BitXor, Div, Mul, Neg, Rem, Sub};
use std::fmt::Display;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Int(i128),
    Float(f64),
    NaN
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Number::Int(val) => write!(f, "{val}"),
            &Number::Float(val) => write!(f, "{val}"),
            &Number::NaN => write!(f, "NaN")
        }
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(i) => i as f64, //TODO: do safe conversion here
            Number::Float(f) => f,
            Number::NaN => f64::NAN
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        if value == std::f64::NAN || value == std::f64::INFINITY || value == std::f64::NEG_INFINITY {
            Number::NaN
        } else {
            Number::Float(value)
        }

    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::Int(value)
    }
}

impl From<Option<i128>> for Number {
    fn from(value: Option<i128>) -> Self {
        match value {
            Some(i) => Number::Int(i),
            None => Number::NaN
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::NaN
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        match self {
            Number::NaN => Number::NaN,
            Number::Int(l) => l.checked_neg().into(),
            _ => {
                let l: f64 = self.into();
                (-l).into()
            }
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::Int(l), Number::Int(r)) => l.checked_add(r).into(),
            _ => {
                let l: f64 = self.into();
                let r: f64 = rhs.into();
                (l + r).into()
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::Int(l), Number::Int(r)) => l.checked_sub(r).into(),
            _ => {
                let l: f64 = self.into();
                let r: f64 = rhs.into();
                (l - r).into()
            }
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::Int(l), Number::Int(r)) => l.checked_mul(r).into(),
            _ => {
                let l: f64 = self.into();
                let r: f64 = rhs.into();
                (l * r).into()
            }
        }
    }
}


impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::Int(l), Number::Int(r)) => l.checked_rem(r).into(),
            _ => {
                let l: f64 = self.into();
                let r: f64 = rhs.into();
                l.rem_euclid(r).into()
            }
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::Int(l), Number::Int(r)) => {
                let rem = l.checked_rem(r);
                match rem {
                    Some(r) if r > 0 => Number::Float(self.into()) / Number::Float(rhs.into()),
                    Some(_) => Number::Int(l / r),
                    None => Number::NaN
                }
            },
            _ => {
                let l: f64 = self.into();
                let r: f64 = rhs.into();
                (l / r).into()
            }
        }
    }
}

// BitXor (the ^ operator) is used as exponent, so 2^3 is 2 pow 3
impl BitXor for Number {
    type Output = Number;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::NaN, _) | (_, Number::NaN) => Number::NaN,
            (Number::Int(l), Number::Int(r)) => {
                if let Ok(exp) = r.try_into() {
                    l.checked_pow(exp).into()
                } else {
                    Number::NaN
                }
                
            },
            _ => {
                let l: f64 = self.into();
                let r: f64 = rhs.into();
                l.powf(r).into()
            }
        }
    }
}
