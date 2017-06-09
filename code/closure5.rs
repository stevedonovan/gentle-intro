// closure5.rs

struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {val: x1, end: x2, incr: skip}
}
    

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}
fn main() {

    let sine = range(0.0,1.0,0.1).map(|x| x.sin()).collect::<Vec<f64>>();

    let sum: f64 = range(0.0,1.0,0.1).map(|x| x.sin()).sum();

    println!("{:?} {}",sine,sum);

    let tuples = [(10,"ten"),(20,"twenty"),(30,"thirty"),(40,"forty")];
    let ti = tuples.iter();
    //~ let iter = ti.filter(|t| t.0 > 20).map(|t| t.1);

    let iter = ti.filter(|&&(num,_)| num > 20).map(|&(_,name)| name);

    for name in iter{
        print!("{} ",name);
    }
    println!("");

    //nums.iter().zip(names.iter())
    
    
}
