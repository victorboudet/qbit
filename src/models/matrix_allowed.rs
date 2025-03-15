use std::fmt;

use super::complex::Complex;

#[derive(Debug)]
pub enum MatrixAllowed {
    Complex(Complex),
    Float(f64),
}

impl fmt::Display for MatrixAllowed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MatrixAllowed::Complex(n) => write!(f, "{}", n),
            MatrixAllowed::Float(n) => write!(f, "{}", n),
        }
    }
}
