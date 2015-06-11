fn factorial(k: u64) -> u64 {
    if k <= 1 {
        return 1;
    }
    return k*factorial(k-1);
}

fn partial_sum(x: f64, n: u32) -> f64 {
    let mut x_1: f64 = 1.0;
    let mut result: f64 = 0.0;
    let mut sign: f64 = 1.0;

    for i in 0..n {
        result += (sign as f64*x_1) / factorial(2*i) as f64;
        sign *= -1;
        x_1 = x_1*x*x;
    }
    return result;
}

fn main() {
    println!("{}", partial_sum(3.14, 5));
}
