fn main() {
    println!("Tables de multiplication");

    for i in 2..11 {
        for j in 1..11 {
            println!("{} * {} = {}", j, i, j*i);
        }
    }
}
