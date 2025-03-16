use std::{fmt, ops};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    pub fn from_float(real: f64) -> Self {
        Self { real, imag: 0.0 }
    }
    pub fn conjugate(self) -> Complex {
        Complex::new(self.real, -self.imag)
    }
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
    pub fn theta(&self) -> f64 {
        (self.imag / self.real).atan()
    }
    pub fn dump(&self) {
        println!("{}", self);
    }
}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl ops::Add<f64> for Complex {
    type Output = Complex;

    fn add(self, other: f64) -> Complex {
        Complex {
            real: self.real + other as f64,
            imag: self.imag,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + other.real * self.imag,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag == 0.0 {
            return write!(f, "{}", self.real);
        }
        if self.imag < 0.0 {
            return write!(f, "{} - {}i", self.real, -self.imag);
        }
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn new() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.real, 3.0);
        assert_eq!(c.imag, 4.0);
    }

    #[test]
    fn conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conjugate();
        assert_eq!(conj.real, 3.0);
        assert_eq!(conj.imag, -4.0);
    }

    #[test]
    fn magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn theta() {
        let c = Complex::new(1.0, 1.0);
        assert_eq!(c.theta(), PI / 4.0);
        let c = Complex::new(0.0, 1.0);
        assert_eq!(c.theta(), PI / 2.0);
        let c = Complex::new(1.0, 0.0);
        assert_eq!(c.theta(), 0.0)
    }

    #[test]
    fn add_complex() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let result = c1 + c2;
        assert_eq!(result.real, 4.0);
        assert_eq!(result.imag, 6.0);
    }

    #[test]
    fn add_f64() {
        let c = Complex::new(1.0, 2.0);
        let result = c + 3.0;
        assert_eq!(result.real, 4.0);
        assert_eq!(result.imag, 2.0);
    }

    #[test]
    fn mul() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = Complex::new(4.0, -8.0);
        let res = c1 * c2;
        assert_eq!(res.real, 32.0);
        assert_eq!(res.imag, -4.0)
    }
    #[test]
    fn mul_conjugate() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = c1.clone().conjugate();
        let res = c1 * c2;
        assert_eq!(res.real, 13.0);
        assert_eq!(res.imag, 0.0)
    }
}
