use models::complex::Complex;

pub mod models;

fn main() {
    let c1: Complex = Complex::new(1.2, 2.3);
    let c2: Complex = Complex::new(-1.0, 0.0);
    println!("atan {}", c2.theta());
    let c3: Complex = c1 + c2;
    c3.dump();
    
}
