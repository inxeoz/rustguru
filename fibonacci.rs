pub fn fib(n: usize){
    let mut a = 0;
    let mut b = 1;
    for i in 0..n+1{
        print!(" {a} ");
        let c = b;
        b = b+a;
        a = c;

    }
}