#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

impl Direction {
    
    pub fn to_x(&self) -> i8 {
        match *self {
            Direction::RIGHT => 1,
            Direction::LEFT => -1,
            _ => 0,
        }
    }

    pub fn to_y(&self) -> i8 {
       match *self {
            Direction::UP => -1,
            Direction::DOWN => 1,
            _ => 0,
       }
    }
}
