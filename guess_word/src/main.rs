extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut count = 0;

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        println!("You guessed : {}", guess);
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less      => {
                println!("Too small !");
                count = count + 1;
            },
            Ordering::Greater   =>  {
                println!("Too big !");
                count = count + 1;
            },
            Ordering::Equal     =>  {
                println!("You win in {} tries!", count);
                break;
            }
        }
    }
}
