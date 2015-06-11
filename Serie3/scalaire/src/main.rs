use std::io;

fn scalaire(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        panic!("Vector must be of same size");
    }
    let mut result: f64 = 0.0;
    for i in 0..a.len() {
        result +=  a[i]*b[i];
    }
    return result;
}

fn main() {
    const MAX: usize = 10;
    let mut input = io::stdin();
    let mut buffer = String::new();

    println!("Enter the vector size (<{})", MAX);
    input.read_line(&mut buffer).unwrap();
    let vector_size: usize = buffer.trim().parse().unwrap();
    let mut a = [0.0; MAX];
    let mut b = [0.0; MAX];

    if vector_size > MAX {
        panic!("The size is greater than {}", MAX);
    }
    
    println!("Enter values for the first vector");

    for i in 0..vector_size {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        a[i] = buffer.trim().parse().unwrap();
    }

    println!("Enter the values for the second vector");

    for i in 0..vector_size {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        b[i] = buffer.trim().parse().unwrap();
    }

    let r = scalaire(&a, &b);
    println!("{}", r);
}
