// iter4.rs

fn main() {
    let mut vec = vec!["one".to_string(),"two".to_string()];

    for n in vec.iter().map(|x: &String| x.len()) {
        println!("{:?}",n);
    }

    for s in vec.iter().filter(|x: &&String| x.len() > 2) {
        println!("{:?}",s);
    }
    
    for s in vec.iter().filter(|x: &&String| *x == "one") {
        println!("{:?}",s);
    }

    for s in vec.iter().filter(|&x| x == "one") {
        println!("{:?}",s);
    }

    let tuples = [("hello",1),("dolly",2)];
    for s in tuples.iter().filter(|(&name,&num)| name == "one") {
        println!("{:?}",s);
    }

    //~ for s in &vec { println!("{:?}",s); }
    //~ for s in &mut vec { println!("{:?}",s); }
    //~ for s in vec {  println!("{:?}",s);  }

    
}


