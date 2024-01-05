pub fn a2z(){
    for  i in 0..26u8 {
        println!("{}", char::from(i + 65u8));
    }
}