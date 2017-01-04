// match3.rs
fn match_tuple(t: (i32,String)) {
    let text = match t {
        (0, s) => format!("zero {}", s),
        (1, s) if s == "hello" => format!("hello one!"),
        tt => format!("no match {:?}", tt)
     };
    println!("{}", text);
}

fn match_maybe_tuple(ot: Option<(i32,String)>) {
    if let Some((_,ref s)) = ot {
        println!("{}", s);
    }
    
    let text = match ot {
        Some((n,ref s)) if s == "hello" => format!("hello {}!", n),
        _ => format!("no match")
    };
    println!("{}", text);
    
}

fn s(text: &str, n: i32) -> (i32,String) {
    (n,text.to_string())
}

fn main() {
    match_tuple(s("hello",0));
    match_tuple(s("hello",1));
    match_tuple(s("world",42));

    let t = s("hello",42);
    match_maybe_tuple(Some(t));

    match (42,"answer") {
        (42,"answer") => println!("yes"),
        _ => println!("no")
    };
}
