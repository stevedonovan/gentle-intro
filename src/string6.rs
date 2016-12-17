// string5.rs
fn array_to_str(arr: &[i32]) -> String {
    let mut res = "[".to_string();    
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let arr = array_to_str(&[10,20,30]);
    let res = format!("hello {}",arr);
    
    assert_eq!(res,"hello [10,20,30]");
}
