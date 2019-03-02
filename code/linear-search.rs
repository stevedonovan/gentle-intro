fn main(){
    another_function(2)
}


fn another_function(x: usize){

    let nums = vec![1,2,3];
    for i in 0..nums.len(){
        if i == x {
            println!("number:{}, index: {}",nums[i] , i);
        }
    }
}
