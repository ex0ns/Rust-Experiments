use super::rawlink;
use super::direction;

pub struct Segment {
    pub dir: direction::Direction,
    pub size: usize,
    pub next: Option<Box<Segment>>,
    pub prev: rawlink::RawLink<Segment>,
}

impl Segment {

    pub fn new() -> Segment {
        Segment { dir: direction::Direction::RIGHT, size: 1, next: None, prev: rawlink::RawLink::none() }
    }

    pub fn increase(&mut self) {
        self.size += 1;
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
