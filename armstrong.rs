pub fn arm(n: usize) -> (bool, usize){

    let mut s = 0usize;
    for i in n.to_string().chars(){
        let v = i as usize - '0' as usize;
       // println!("{v} {}", v.pow(n.to_string().len() as u32));
        s += v.pow(n.to_string().len() as u32);
    }
    if s == n{
        return (true, n);
    }
    (false, n)
}
