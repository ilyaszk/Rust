enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    move_avatar(Direction::Up);
    move_avatar(Direction::Down);
    move_avatar(Direction::Left);
    move_avatar(Direction::Right);
}

fn move_avatar(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}
