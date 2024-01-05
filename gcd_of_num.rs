
pub fn gcd( num: &mut [usize]){
    let mut fil: Vec<usize> = num.iter().filter( |&x| *x != 0).cloned().collect();
    if fil.len() == 0{
        return;
    }
    fil.sort();
    let g = fil[0];
    if fil.len() == 1 || g == 1{
        println!("got gcd {}", g);
    }else{
        let mut fl2: Vec<usize> = fil.iter().map(|x| *x % g).collect();
        println!("{:?}", fil);
        gcd(&mut fl2.clone());
    }

}