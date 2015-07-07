mod network;

fn main() {
    let mut net = network::Network::new(3, vec![2, 3, 1]);
    net.sgd(vec![(1.0f64, 1.0f64), (2.0f64, 2.0f64), (3.0f64, 3.0f64)], 4, 2, 4.0);
    println!("Hello, world!");
}
