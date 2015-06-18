mod snake;
mod segment;
mod direction;
mod rawlink;
mod map;

fn main() {
    let mut map = map::Map::new(20, 20);
    let mut snake : snake::Snake = snake::Snake::new();
    for _i in 0..2 {
        snake.step(&mut map, direction::Direction::DOWN);
    }
    map.add_snake(&snake); 
    println!("{}", map);
}
