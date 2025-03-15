use std::ops;

use super::matrix_allowed::MatrixAllowed as Allowed;

#[derive(Debug)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    pub numbers: Vec<Allowed>,
}

impl Matrix {
    pub fn new(n: usize, m: usize, numbers: Vec<Allowed>) -> Result<Self, &'static str> {
        if n * m != numbers.len() {
            return Err("The number of elements does not match the matrix dimensions");
        }
        Ok(Self { n, m, numbers })
    }
    pub fn dump(&self) {
        for i in 0..self.n * self.m {
            if i % self.n == 0 {
                println!();
            }
            print!("{} ", self.numbers[i]);
        }
        println!();
    }
}

impl ops::Add<Matrix> for Matrix {
    type Output = Result<Matrix, String>;

    fn add(self, other: Matrix) -> Result<Matrix, String> {
        if self.m != other.m || self.n != other.n {
            return Err("Matrix dimensions are not the sames".to_string());
        }
        let mut numbers = vec![];
        for i in 0..self.n * self.m {
            numbers.push(todo!());
        }
        Ok(Matrix {
            n: self.n,
            m: self.m,
            numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::complex::Complex;

    use super::*;

    #[test]
    fn new() {
        let c = Complex::new(1.2, 4.3);
        let m = Matrix::new(
            2,
            2,
            vec![
                Allowed::Float(3.0),
                Allowed::Float(3.0),
                Allowed::Float(4.0),
                Allowed::Complex(c),
            ],
        );
        match m {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }
}
