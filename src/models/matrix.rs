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
