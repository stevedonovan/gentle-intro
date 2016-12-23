// tuple2.rs
fn main() {
    for (i,s) in ["zero","one","two"].iter().enumerate() {
        print!(" {} {};",i,s);
    }
    println!("");

    let names = ["ten","hundred","thousand"];
    let nums = [10,100,1000];
    for (name,num) in names.iter().zip(nums.iter()) {
        print!(" {} {};",name,num);
    }
    println!("");
}
