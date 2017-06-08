// closure1.rs

fn main() {
    let f = |x| x*x;

    let res = f(10);

//    let resf = f(1.2);  // error, must be integer arg!

    println!("res {}",res);


}

