use models::complex::Complex;

pub mod models;

fn main() {
    let c1: Complex = Complex::new(1.0, 2.0);
    let c2: Complex = Complex::new(2.3, 1.2);
    let c3: Complex = c1 + c2 + 2.3;
    let c4 = c3.conjugate();
    c4.dump();
}
