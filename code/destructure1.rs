// destructure1.rs

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

fn main() {
    let p = Point{x:1.0,y:2.0};

    let Point{x,y} = p;

    println!("{} {}",x,y);

    println!("{:?}",p);

}
