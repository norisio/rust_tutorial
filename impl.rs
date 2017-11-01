struct Circle {
    x: f64,
    y: f64,
    rad: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.rad * self.rad)
    }
}


fn main() {
    let c = Circle { x: 0.0, y: 0.0, rad:2.0 };
    println!("{}", c.area());
}
