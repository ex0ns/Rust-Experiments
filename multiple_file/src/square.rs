use super::point;

pub struct Square {
    pub top_left: point::Point,
    bottom_right: point::Point,
    pub width: u32,
    height: u32,
}

impl Square {
    pub fn new(first: point::Point, second: point::Point, width: u32, height: u32) -> Square {
        Square { top_left: first, bottom_right: second, width: width, height: height }
    }
}

pub fn test() {
    println!("Ok");
}
