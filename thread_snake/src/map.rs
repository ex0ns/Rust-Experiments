use super::snake;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
enum MapObject {
    EMPTY,
    FOOD,
    SNAKE,
}

pub struct Map {
    width: usize,
    height: usize,
    map: Vec<MapObject>,
}


impl fmt::Display for MapObject {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            MapObject::EMPTY =>  "*",
            MapObject::FOOD => "-",
            MapObject::SNAKE => "#",
        };
        write!(f, "{}", p)
    }

}

impl Map {

    pub fn new(width: usize, height: usize) -> Map {
        let data = vec![MapObject::EMPTY; width*height];
        Map { width: width, height: height, map: data }
    }

    fn at(&self, x: usize, y: usize) -> MapObject {
        let case =  y*self.width + x; 
        if case >= self.width*self.height {
            panic!("Coordinate are out of the map");
        }
        return self.map[case];
    }

    pub fn is_food(&self, x: i32, y: i32) -> bool {
        self.at(x as usize, y as usize) == MapObject::FOOD
    }

    fn set(&mut self, x: usize, y: usize, o: MapObject) {
        let case =  y*self.width + x; 
        if case >= self.width*self.height {
            panic!("Coordinate are out of the map");
        }
        self.map[case] = o;
    }

    pub fn set_snake_head(&mut self, snake: &mut snake::Snake) {
        match snake.head {
            Some(ref head) => {
                self.set(head.x as usize, head.y as usize, MapObject::SNAKE);
            }
            None => panic!("Snake should not be empty")
        }
    }

    pub fn move_snake_tail(&mut self, x: i32, y: i32) {
        self.set(x as usize, y as usize, MapObject::EMPTY);
    }

    pub fn add_snake(&mut self, snake: &snake::Snake) {
        let mut current = &snake.head;
        loop {
           match *current {
                Some(ref segment) => {
                    let mut x = segment.x;
                    let mut y = segment.y;
                    for _i in 0..segment.size {
                        self.set(x as usize, y as usize, MapObject::SNAKE);
                        x += -1*segment.dir.to_x() as i32;
                        y += -1*segment.dir.to_y() as i32;
                    }
                    current = &segment.next;
                }
                None => break
           };
        }
    }
}

impl fmt::Display for Map {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            let line = i*self.width;
            for j in 0..self.width {
                try!(write!(f, "{} ", self.map[line+j]));
            }
            try!(writeln!(f, ""));
        }
        Ok(())
    }

}
