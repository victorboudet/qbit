use std::ops;

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    pub fn conjugate(&mut self) {
        self.imag = -self.imag;
    }
    pub fn dump(self) {
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
