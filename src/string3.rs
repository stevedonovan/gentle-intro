// string3.rs
fn main() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ",ch);
    }
    println!("");
    println!("len {}",multilingual.len());
    println!("count {}",multilingual.chars().count()) ;
    

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}",hi);
    }

    match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}",hi);
        },
        None => println!("couldn't find the greeting, Товарищ")
    };

    if let Some(idx) = multilingual.find('п') {
        println!("Russian hi {}",&multilingual[idx..]);
    }
    
}
