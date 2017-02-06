// cli.rs
use std::io;
use std::collections::HashMap;

type CliResult = Result<String,String>;

struct Cli<'a> {
    callbacks: HashMap<String, Box<FnMut(&[&str])->CliResult + 'a>>
}

impl <'a> Cli<'a> {
    fn new() -> Cli<'a> {
        Cli{callbacks: HashMap::new()}
    }

    fn cmd<F>(&mut self, name: &str, callback: F)
    where F: FnMut(&[&str])->CliResult + 'a {
        self.callbacks.insert(name.to_string(),Box::new(callback));
    }

    fn process(&mut self,line: &str) -> CliResult {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 0 { return Ok("".to_string()); }
        match self.callbacks.get_mut(parts[0]) {
            Some(callback) => callback(&parts[1..]),
            None => Err("no such command".to_string())
        }
    }

    fn go(&mut self) {
        let mut buff = String::new();
        while io::stdin().read_line(&mut buff).expect("error") > 0 {
            {
                let line = buff.trim_left();
                let res = self.process(line);
                println!("{:?}",res);
                
            }
            buff.clear();
        }
    }

}

fn ok<T: ToString>(s: T) -> CliResult {
    Ok(s.to_string())
}

fn err<T: ToString>(s: T) -> CliResult {
    Err(s.to_string())
}


use std::error::Error;

fn main() {
    println!("Welcome to the Interactive Prompt! ");

    let mut answer = 0;

    let mut cli = Cli::new();

    cli.cmd("go",|args| {
        if args.len() == 0 { return err("need 1 argument"); }
        answer = match args[0].parse::<i32>() {
            Ok(n) => n,
            Err(e) => return err(e.description())
        };
        println!("got {:?}", args);
        ok(answer)
    });

    cli.cmd("show",|args| {
        ok(answer)
    });

    cli.go();
}
