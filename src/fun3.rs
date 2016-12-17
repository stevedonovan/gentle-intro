// fun3.rs

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn fact(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n*fact(n-1)
    }
}

fn main() {
    let res1 = abs(-10.0);
    let res2 = clamp(1.5,0.0,1.0);
    let res3 = fact(4);
    println!("res1 is {}",res1);
    println!("res2 is {}",res2);
    println!("res3 is {}",res3);
    
}
