// enum4.rs

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Arr(Vec<Value>)
}

fn parse(txt: &str) -> Value {
    

}

fn main() {
    use Value::*;

    let s = "hello".to_string();
    let v = vec![Number(1.0),Bool(false),Str(s)];
    let arr = Arr(v);

    let res = Arr(vec![Number(2.0),arr]);

    println!("{:#?}",res);
}
