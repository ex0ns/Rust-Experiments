use super::rawlink;
use super::direction;

pub struct Segment {
    pub dir: direction::Direction,
    pub size: usize,
    pub x: i32,
    pub y: i32,
    pub next: Option<Box<Segment>>,
    pub prev: rawlink::RawLink<Segment>,
}

impl Segment {

    pub fn new() -> Segment {
        Segment { dir: direction::Direction::RIGHT, size: 1, x: 0, y: 0, next: None, prev: rawlink::RawLink::none() }
    }

    pub fn increase(&mut self) {
        self.size += 1;
    }

    pub fn update(&mut self) {
        self.x += self.dir.to_x() as i32;
        self.y += self.dir.to_y() as i32;
    }

    pub fn decrease(&mut self) -> rawlink::RawLink<Segment> {
        self.size -= 1;
        if self.size == 0 {
            match self.prev.resolve() {
                Some(s) => {
                    s.next = None;
                    return rawlink::RawLink::some(s);
                }
                None => { return rawlink::RawLink::none(); }, 
            };
        } else {
            return rawlink::RawLink::some(self);
        }
    }
}
