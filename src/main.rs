use models::complex::Complex;

pub mod models;

fn main() {
    let c1: Complex = Complex::new(1.2, 2.3);
    let c2: Complex = Complex::new(-2.0, 3.0);
    println!("magnitude {}", c2.magnitude());
    let c3: Complex = c1 + c2;
    c3.dump();
}
