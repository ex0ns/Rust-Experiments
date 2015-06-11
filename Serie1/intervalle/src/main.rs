use std::io;

fn main() {
    let min:f32 = -1.0;
    let max:f32 = 1.0;
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let x:f32 = input.trim().parse()
        .ok()
        .expect("Please enter a number");

    if x >= min && x < max {
        println!("{} is in [{},{})", x, min, max);
    } else {
        println!("{} is not in [{},{})", x, min, max);
    }
}
