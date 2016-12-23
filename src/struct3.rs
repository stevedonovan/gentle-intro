// struct2.rs

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}",self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        Self::new(&self.first_name,&self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }

}

fn main() {
    let p = Person::new("John","Smith");
    println!("person {} {}",p.first_name,p.last_name);
    
    println!("{:?}",p);

    println!("fullname {}",p.full_name());

    let mut p = Person::new("Alice","Jones");
    p.set_first_name("Alicia");
    let (first,last) = p.to_tuple();
    println!("first {} last {}",first,last);
}
