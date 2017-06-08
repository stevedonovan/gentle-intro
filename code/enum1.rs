// enum1.rs

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right"
        }
    }

    fn inc(&self) -> Direction {
        use Direction::*;
        match *self {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up
        }
    }
}

fn main() {
    let start = Direction::Left;

    assert_eq!(start,Direction::Left);
    
    println!("start {}",start.as_str());
    println!("start {:?}",start);

    let mut d = start;
    for _ in 0..8 {
        println!("d {:?}",d);
        d = d.inc();
    }

    
}
