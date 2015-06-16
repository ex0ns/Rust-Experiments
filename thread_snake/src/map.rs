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
        let mut data = vec![MapObject::EMPTY; width*height];
        Map { width: width, height: height, map: data }
    }

    pub fn at(&self, x: usize, y: usize) -> MapObject {
        let case =  y*self.width + x; 
        if case >= self.width*self.height {
            panic!("Coordinate are out of the map");
        }
        return self.map[case];
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn set(&mut self, x: usize, y: usize, o: MapObject) {
        let case =  y*self.width + x; 
        if case >= self.width*self.height {
            panic!("Coordinate are out of the map");
        }
        self.map[case] = o;
    }

    pub fn add_snake(&mut self, snake: snake::Snake) {
        let mut x = snake.x;
        let mut y = snake.y;
        let mut current = snake.head;
        loop {
           match current {
                Some(segment) => {
                    for _i in 0..segment.size {
                        self.set(x as usize, y as usize, MapObject::SNAKE);
                        x += segment.dir.to_x() as i32;
                        y += segment.dir.to_y() as i32;
                    }
                    current = segment.next;
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
