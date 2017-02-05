// error1.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }   
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}


impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}


impl From<std::num::ParseFloatError> for MyError {
    fn from(err: std::num::ParseFloatError) -> Self {
        MyError::new(err.description())
    }
}

fn raises_my_error(yes: bool) -> Result<(),MyError> {
    if yes {
        Err(MyError::new("borked"))
    } else {
        Ok(())
    }
}

fn parse_f64(s: &str, yes: bool) -> Result<f64,MyError> {
    raises_my_error(yes)?;
    let x: f64 = s.parse()?;
    Ok(x)
}

fn main() {
    println!(" {:?}",parse_f64("42",false));
    println!(" {:?}",parse_f64("42",true));
    println!(" {:?}",parse_f64("?42",false));
}
