struct Complexe {
    a: f64,
    b: f64,
}

fn print(c: &Complexe) {
    println!("{} + i*{}", c.a, c.b);
}

fn add(c1: &Complexe, c2: &Complexe) -> Complexe {
    return Complexe { a: c1.a + c2.a, b: c1.b + c2.b };        
}

fn inverse(c1: &Complexe) -> Complexe {
    return Complexe { a: c1.a*-1.0, b: c1.b*-1.0};
}

fn sub(c1: &Complexe, c2: &Complexe) -> Complexe {
    return add(c1, &inverse(c2));
}

fn main() {
    let c = Complexe { a: 12.3, b:1.0 };
    print(&sub(&c, &c));
}
