use std::cmp::{max, min};

pub fn lcm(num:&mut [usize]){
    let mut fil: Vec<usize>= num.clone();
    fil.sort();
    let c = a*b;
    let mut lcm = c;
    for i in max(a, b)..c {
        if i%a ==0 && i%b ==0{
            lcm = i;
            break;
        }
    }
    println!("lcm {lcm}");

}