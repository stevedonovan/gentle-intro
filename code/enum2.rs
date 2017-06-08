// enum2.rs

#[derive(PartialEq,PartialOrd)]
enum Speed {
    Slow = 10,
    Medium, // = 20,
    Fast// = 50
}

fn main() {
    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}",speed);
    println!("fast {}",Speed::Fast as u32);

    assert!(Speed::Fast > Speed::Slow);
}
