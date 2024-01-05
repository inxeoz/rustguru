use std::cmp;
pub fn max(numbers: &[i32]){
    let l = numbers.len();
    if l !=0{
        let mut c = numbers[0];
        for num in numbers.iter().skip(1){
            c = cmp::max(c, *num);
        }
        println!("max {c}");
    }else {
        eprint!("not found max");
    }
}

