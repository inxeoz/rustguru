pub fn fac(a:i32){
    let n = a.abs();
    print!(" {n}");

    for i in 1..=n {
        if n%i ==0{
            print!(" {i} ");
            if a<0{
                print!( " -{i} ");
            }

        }
    }
}