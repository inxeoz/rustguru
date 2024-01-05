pub fn fac(a: usize) -> usize{
    if a < 2{
        1
    }else {
        a*fac(a-1)
    }
}