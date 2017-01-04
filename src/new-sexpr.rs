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
    current: Vec<Value>,
    error: Option<String>,
    open: bool
}

impl Builder {
    fn new() -> Builder {
        Builder {
            stack: Vec::new(),
            current: Vec::new(),
            error: None,
            open: false
        }
    }


    fn push(&mut self, v: Value) -> &mut Builder {
        if ! self.open {
            self.error = Some("not open!".to_string());
        }
        if self.error.is_none() {
            self.current.push(v);
        }
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

    fn value(&mut self) -> Result<Value,String> {
        match self.error {
            None => {
                let current = self.extract_current(Vec::new());
                Ok(Value::Arr(current))
            },
            Some(ref s) => Err(s.clone())
        }
    }
    

    fn open(&mut self) -> &mut Builder {
        if ! self.open {
            self.open = true;
            return self;
        }
        if self.error.is_some() { return self; }
        let current = self.extract_current(Vec::new());
        self.stack.push(current);
        self
    }
    
    fn close(&mut self) -> &mut Builder {
        if let Some(last_current) = self.stack.pop() {
            let current = self.extract_current(last_current);
            self.current.push(Value::Arr(current));
        } else {
            if self.open {
                self.open = false;
            } else {
                self.error = Some("mismatched open/close".to_string());
            }
        }
        self
    }


}


fn parse(text: &str) -> Result<Value,String> {
    let mut builder = Builder::new();
    let mut word = String::new();
    for ch in text.chars() {
        if ch.is_whitespace() {            
            if word.len() > 0 {
                parse_word(&mut builder, &word)?;
                word.clear();
            }            
        } else
        if ch == '(' {
            builder.open();
        } else
        if ch == ')' {
            if word.len() > 0 {
                parse_word(&mut builder, &word)?;
                word.clear();
            }            
            builder.close();
        } else {
            word.push(ch);
        }
    }
    builder.value()
}

use std::error::Error;

fn parse_word(builder: &mut Builder, word: &str) -> Result<(),String> {
    // guaranteed to be at least one character!
    let first = word.chars().next().unwrap();
    if word == "T" || word == "F" {
        builder.b(word == "T");
    } else
    if first.is_digit(10) || first == '-' {
        match word.parse::<f64>() {
        Ok(num) => builder.n(num),
        Err(err) => return Err(err.description().to_string())
        };
    } else {
        builder.s(&word);
    }
    Ok(())
}

struct Pairs<'a> {
    slice: &'a [Value],
    idx: usize
}

fn pairs(v: &Value) -> Option<Pairs> {
    match *v {
        Value::Arr(ref arr) => Some(Pairs {
            slice: &arr,
            idx: 0
        }),
        _ => None
    }
}

impl <'a> Iterator for Pairs<'a> {
    type Item = (&'a str, &'a Value);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.slice.len() {
            return None; // no more pairs
        }
        let v = &self.slice[self.idx];
        self.idx += 1;
        match *v {
            Value::Arr(ref arr) if arr.len() > 2 => {
                match arr[0] {
                    Value::Str(ref s) => {
                        Some((s,&arr[1]))
                    }
                    _ => None
                }
            },
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct SexprError {
    details: String
}

impl fmt::Display for SexprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl SexprError {
    pub fn new(msg: &str) -> SexprError {
        SexprError{details: msg.to_string()}
    }
    
    pub fn err<T>(msg: String) -> Result<T,SexprError> {
        Err(SexprError { details: msg })
    }
}

impl Error for SexprError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::num::ParseFloatError> for SexprError {
    fn from(err: std::num::ParseFloatError) -> Self {
        SexprError::new(err.description())
    }
}


fn eval(v: &Value) -> Result<f64,SexprError> {
    match *v {
        Value::Arr(ref arr) if arr.len() > 2 => {
            match arr[0] {
                Value::Str(ref s) => {
                    if s == "+" || s == "*" {
                        let adding = s == "+";
                        let mut res = if adding {0.0} else {1.0};
                        for v in &arr[1..] {
                            let num = eval(v)?;
                            res = if adding {
                                res + num
                            } else {
                                res * num
                            }
                        }
                        Ok(res)
                    } else
                    if s == "-" || s == "/" {
                        let x = eval(&arr[1])?;
                        let y = eval(&arr[2])?;
                        let res = if s == "-" {
                            x - y
                        } else {
                            x / y
                        };
                        Ok(res)
                    } else {
                        SexprError::err(format!("unknown operator {:?}", s))
                    }
                },
                ref v => SexprError::err(format!("operator must be string {:?}", v))
            }
        },
        Value::Number(x) => Ok(x),
        ref v => SexprError::err(format!("cannot convert {:?} to number", v))
    }
}

fn main() {

    // building the hard way
    /*
    use Value::*;

    let s = "hello".to_string();
    let v = vec![Number(1.0),Bool(false),Str(s)];
    let arr = Arr(v);
    let res = Arr(vec![Number(2.0),arr]);    

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
    .close().close().value().expect("error");
    
    println!("{:?}",res);
    println!("{}",res);
    */

    let default = "( (one 1) (two 2) (three 3) )";
    let test = std::env::args().skip(1).next().unwrap_or(default.to_string());
    let res = parse(&test).expect("error");

    println!("{:?}",res);
    println!("{}",res);

    //~ for (s,e) in pairs(&res).expect("iter") {
        //~ println!("{} {}",s,e);
    //~ }

    let x = eval(&res);
    println!("result is {:?}",x);
    
}
