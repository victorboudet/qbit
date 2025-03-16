use std::ops;

use super::complex::Complex;

#[derive(Debug)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    pub numbers: Vec<Complex>,
}

impl Matrix {
    pub fn new(n: usize, m: usize, numbers: Vec<Complex>) -> Result<Self, &'static str> {
        if n * m != numbers.len() {
            return Err("The number of elements does not match the matrix dimensions");
        }
        Ok(Self { n, m, numbers })
    }
    pub fn dump(&self) {
        if self.numbers.is_empty() {
            println!("Empty matrix");
            return;
        }
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
            numbers.push(other.numbers[i] + self.numbers[i]);
        }
        Ok(Matrix {
            n: self.n,
            m: self.m,
            numbers,
        })
    }
}

impl ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut numbers = vec![];
        for i in other.numbers {
            numbers.push(i * Complex::from_float(self));
        }
        Matrix {
            n: other.n,
            m: other.m,
            numbers,
        }
    }
}

impl ops::Mul for Matrix {
    type Output = Result<Matrix, String>;

    fn mul(self, other: Matrix) -> Result<Matrix, String> {
        if self.m != other.n {
            return Err("Undefined".to_string());
        }
        let mut numbers = vec![];
        for i in 0..self.n {
            for j in 0..other.m {
                let mut res = Complex::from_float(0.0);
                for k in 0..self.m {
                    res = self.numbers[i * self.m + k] * other.numbers[k * other.m + j] + res;
                }
                numbers.push(res);
            }
        }
        Ok(Matrix {
            n: self.n,
            m: other.m,
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
                Complex::from_float(2.0),
                Complex::from_float(5.0),
                c,
                Complex::from_float(9.0),
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

    #[test]
    fn mul_two_matrices() {
        let m1 = Matrix::new(
            2,
            3,
            vec![
                Complex::from_float(1.0),
                Complex::from_float(2.0),
                Complex::from_float(3.0),
                Complex::from_float(4.0),
                Complex::from_float(5.0),
                Complex::from_float(6.0),
            ],
        )
        .expect("It should work");
        let m2 = Matrix::new(
            3,
            2,
            vec![
                Complex::from_float(7.0),
                Complex::from_float(8.0),
                Complex::from_float(9.0),
                Complex::from_float(10.0),
                Complex::from_float(11.0),
                Complex::from_float(12.0),
            ],
        )
        .expect("It will work");
        let m3 = (m1 * m2).expect("It should be ok");
        assert_eq!(m3.numbers[0], Complex::from_float(58.0));
        assert_eq!(m3.numbers[1], Complex::from_float(64.0));
        assert_eq!(m3.numbers[2], Complex::from_float(139.0));
        assert_eq!(m3.numbers[3], Complex::from_float(154.0));
    }

    #[test]
    fn mul_matrix_by_factor() {
        let m = Matrix::new(
            3,
            2,
            vec![
                Complex::from_float(7.0),
                Complex::from_float(8.0),
                Complex::from_float(9.0),
                Complex::from_float(10.0),
                Complex::from_float(11.0),
                Complex::from_float(12.0),
            ],
        )
        .expect("It should work");
        let m1 = 2.0 * m;
        assert_eq!(m1.numbers[0], Complex::from_float(14.0));
        assert_eq!(m1.numbers[1], Complex::from_float(16.0));
        assert_eq!(m1.numbers[2], Complex::from_float(18.0));
        assert_eq!(m1.numbers[3], Complex::from_float(20.0));
        assert_eq!(m1.numbers[4], Complex::from_float(22.0));
        assert_eq!(m1.numbers[5], Complex::from_float(24.0));
    }
}
