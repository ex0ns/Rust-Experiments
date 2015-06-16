mod square;
mod point;

fn main() {
    let first = point::Point::new(12, 13);
    let second = point::Point::new(15, 16);
    let square = square::Square::new(first, second, 15-12, 16-13);
    println!("{} {}", square.top_left.x, square.width);
}
