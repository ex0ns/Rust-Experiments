use std::io;

fn main() {
    const G: f32 = 9.81;
    let mut input = String::new();

    println!("Please enter the original height");
    io::stdin().read_line(&mut input).unwrap();
    let h0: f32 = input.trim().parse()
        .ok()
        .expect("Enter a number");
    if h0 < 0.0 {
        panic!("The original height must be greater or equal to 0");
    }
    
    println!("Please enter the coefficient of restitution");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let eps: f32 = input.trim().parse()
        .ok()
        .expect("Enter a number");
    if eps < 0.0 || eps >= 1.0 {
        panic!("The coeff must be between 0 and 1");
    }


    println!("Please enter the number of bounces");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let bounces: u32 = input.trim().parse()
        .ok()
        .expect("Enter a number");

    let mut height = h0;
    let mut speed = (2.0*height*G).sqrt();
    for _i in 0..bounces {
        speed *= eps;
        height = speed*speed / (2.0*G);
    }

    println!("After {} bounces the ball is {} high", bounces, height);
}
