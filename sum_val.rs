pub fn sum(val: &[i32]){
    let mut s = 0;
    for i in val{
        s += *i;

    }
    println!("sum is {s}");
}