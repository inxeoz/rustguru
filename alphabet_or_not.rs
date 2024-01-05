
pub fn alpha(e: char){
    let c = e.to_ascii_lowercase();
    let n = c as u8;
    if n >= 97 && n <= 122{
        println!("alpha {e}");
    }else{
        println!("not alpha {e}");
    }
}