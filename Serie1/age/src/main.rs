use std::io;

fn main() {
    let current_year = 2015;
    let mut input = String::new();

    print!("Input your age: ");
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse()
        .ok()
        .expect("Please type a number");

    let birth_year = current_year - age;
    println!("You are {} and you were born in {}", age, birth_year);
}
