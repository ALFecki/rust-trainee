

pub struct Biquadratic {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64
}

impl Biquadratic {
    pub fn new_from_coeffs(a: f64, b: f64, c: f64, d: f64, e: f64) -> Self {
        Biquadratic { a, b, c, d, e }
    }

    pub fn print(&self) {
        println!("{:+}x^4 {:+}x^3 {:+}x^2 {:+}x {:+}", self.a, self.b, self.c, self.d, self.e);

    }
}