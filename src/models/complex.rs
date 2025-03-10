use std::{f64::consts::PI, ops};

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    pub fn conjugate(self) -> Complex {
        Complex::new(self.real, -self.imag)
    }
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
    pub fn theta(&self) -> f64 {
        if self.real == 0.0 {
            if self.imag > 0.0 {
                return PI / 2.0;

            }
            if self.imag < 0.0 {
                return -PI / 2.0;
            }
            return 0.0;
        }
        (self.real/self.imag).atan()
    }
    pub fn dump(&self) {
        if self.imag < 0.0 {
            println!("{} - {}i", self.real, -self.imag);
        } else {
            println!("{} + {}i", self.real, self.imag);
        }
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
