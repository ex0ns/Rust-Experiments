mod snake;
mod segment;
mod direction;
mod rawlink;
mod map;

fn main() {
    let mut map = map::Map::new(20, 20);
    let mut snake : snake::Snake = snake::Snake::new();
    snake.eat();
    snake.eat();
    map.add_snake(snake); 
    println!("{}", map);
}
