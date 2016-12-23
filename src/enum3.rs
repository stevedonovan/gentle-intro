// enum3.rs

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn dump(v: &Value) {
    use Value::*;
    match *v {
    Number(n) => println!("number is {}",n),
    Str(ref s) => println!("string is '{}'",s),
    Bool(b) => println!("boolean is {}",b)
    }
}

impl Value {
    fn to_number(self) -> Option<f64> {
        match self {
        Value::Number(n) => Some(n),
        _ => None
        }        
    }

    fn to_str(self) -> Option<String> {
        match self {
        Value::Str(s) => Some(s),
        _ => None
        }
    }
}

fn main() {
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}",n,s,b);

    dump(&s);

    println!("s? {:?}",s.to_str());
}
