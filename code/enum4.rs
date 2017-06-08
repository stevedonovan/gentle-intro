// enum4.rs

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Arr(Vec<Value>)
}

use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;
        match *self {
            Number(n) => write!(f,"{} ",n),
            Str(ref s) => write!(f,"{} ",s),
            Bool(b) => write!(f,"{} ",b),
            Arr(ref arr) => {
                write!(f,"(")?;
                for v in arr.iter() {
                    v.fmt(f)?;
                }
                write!(f,")")
            }
        }
    }
}

struct Builder {
    stack: Vec<Vec<Value>>,
    current: Vec<Value>
}

impl Builder {
    fn new() -> Builder {
        Builder {
            stack: Vec::new(),
            current: Vec::new()
        }
    }


    fn push(&mut self, v: Value) -> &mut Builder {
        self.current.push(v);
        self
    }

    fn s(&mut self, s: &str) -> &mut Builder {
        self.push(Value::Str(s.to_string()))
    }

    fn b(&mut self, v: bool) -> &mut Builder {
        self.push(Value::Bool(v))
    }

    fn n(&mut self, v: f64) -> &mut Builder {
        self.push(Value::Number(v))
    }

    fn extract_current(&mut self, arr: Vec<Value>) -> Vec<Value> {
        let mut current = arr;
        std::mem::swap(&mut current, &mut self.current);
        current
    }

    fn value(&mut self) -> Value {
        Value::Arr(self.extract_current(Vec::new()))
    }    
    

    fn open(&mut self) -> &mut Builder {
        let current = self.extract_current(Vec::new());
        self.stack.push(current);
        self
    }
    
    fn close(&mut self) -> &mut Builder {
        let last_current = self.stack.pop().expect("stack empty");
        let current = self.extract_current(last_current);
        self.current.push(Value::Arr(current));
        self
    }


}

fn main() {

    // building the hard way
    use Value::*;

    let s = "hello".to_string();
    let v = vec![Number(1.0),Bool(false),Str(s)];
    let arr = Arr(v);
    let res = Arr(vec![Number(2.0),arr]);
    //*/

    println!("{:?}",res);
    println!("{}",res);    

    let res = Builder::new().open()
    .s("one")
    .open()
        .s("two")
        .b(true)
        .open()
          .s("four")
          .n(1.0)
        .close()
    .close().close().value();
    
    println!("{:?}",res);
    println!("{}",res);
}
