use crate::prime::prm;
pub fn val(a: usize){
    let m = a/2;
    for i in 0..=m{
        let v = a-i;
        if prime::prm(v) && prime::prm(i){
            println!("yes {v} + {i}");
            return;
        }
    }
}