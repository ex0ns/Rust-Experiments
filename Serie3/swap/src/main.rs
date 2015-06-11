fn swap(a : &mut u32, b: &mut u32) {
    let c = *a;
    *a = *b;
    *b = c;
}

fn main() {
    let mut a = 12;
    let mut b = 14;
    swap(&mut a, &mut b);
    println!("a is {} and b {}", a, b);
    
}
