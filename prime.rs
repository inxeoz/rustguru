pub fn prm(n: usize) -> bool {
    for i in 2..n / 2 {
        if n % i == 0 {
            /// print!("not prime {n}");
            return false;
        }
    }
    ///  println!("prime {n}");
    true
}