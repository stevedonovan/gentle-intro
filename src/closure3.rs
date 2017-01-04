// closure3.rs

fn call<F>(ch: char, f: F)
where F: Fn(char)->bool {
    let res = f(ch);
    println!("{}", res);
}

fn mutate<F>(mut f: F)
where F: FnMut() {
    f()
}

fn main() {
    let s = "hello dolly".to_string();

    call('d',|c| s.find(c).is_some());
    call('l',|c| s.find(c).is_some());

    let mut s = "world";
    mutate(|| s = "hello");
    assert_eq!(s,"hello");
}
