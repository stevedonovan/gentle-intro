use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    let data = r#" {
     "name": "John Doe", "age": 43,
     "address": {"street": "main", "city":"Downtown"},
     "phones":["27726550023"]
    } "#;
    let p: Person = serde_json::from_str(data).expect("deserialize error");
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    println!("{:#?}",p);
}
