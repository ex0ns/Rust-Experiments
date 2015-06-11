use std::io;

fn main() {
    const G: f32 = 9.81;
    let mut input = io::stdin();

    let mut buffer = String::new();
    println!("Please enter the original height");
    input.read_line(&mut buffer).unwrap(); 

    let h0: f32 = buffer.trim().parse()
        .ok()
        .expect("The height must be a number");

    buffer.clear();

    println!("Please enter the coefficient of restitution");

    input.read_line(&mut buffer).unwrap();
    let eps: f32 = buffer.trim().parse()
        .ok()
        .expect("Please enter a number");

    if eps < 0.0 || eps >= 1.0 {
        panic!("Coefficient of restitution must be between 0 and 1");
    }

    buffer.clear();

    println!("Enter the final height");

    input.read_line(&mut buffer).unwrap();
    let hf: f32 = buffer.trim().parse()
        .ok()
        .expect("Please enter a number");

    if hf > h0 || hf < 0.0 {
        panic!("The final height must be between 0 and {}", h0);
    }

    let mut height = h0;
    let mut speed = (2.0*G*height).sqrt();
    let mut nb_bounces = 0;
    while height > hf {
        speed *= eps;
        height = speed*speed / (2.0*G);
        nb_bounces += 1;
    }

    println!("Number of bounces: {}", nb_bounces);
}
