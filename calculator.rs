pub fn ope(n: char, a: isize, b:isize){
    match n {
        '+' => println!("{a} + {b} = {}", a+b),
        '-' => println!("{a} - {b} = {}", a-b),
        '/' => println!("{a} / {b} = {:.2} with remainder {}", a/b, a%b),
        '*' => println!("{a} * {b} = {}", a*b),
        _ => println!("invalid operation "),
    }
}