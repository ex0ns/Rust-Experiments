pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }            
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }
}
