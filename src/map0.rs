// map0.rs

fn main() {
    let entries = [("one","eins"),("two","zwei"),("three","drei")];

    if let Some(val) = entries.iter().find(|t| t.0 == "two") {
        assert_eq!(val.1,"zwei");
    }

}
