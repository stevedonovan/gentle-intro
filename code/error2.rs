// error2.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String,
    original_error: Option<Box<Error>>
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string(), original_error: None}
    }

    fn from<E: Error + 'static> (e: E) -> MyError {
        MyError{
            details: e.description().to_string(),
            original_error: Some(Box::new(e))
        }
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

    fn cause(&self) -> Option<&Error> {
        match self.original_error {
            Some(ref err) => Some(&*err),
            None => None
        }
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
