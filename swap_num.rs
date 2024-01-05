

pub fn swap(mut a:i32, mut b:i32){
    println!("pre. {a} {b}");
    let c = a;
    a = b;
    b = c;
    println!("aft. {a} {b}");

}