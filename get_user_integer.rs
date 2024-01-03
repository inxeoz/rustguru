use std::io::stdin;

pub fn run(){
    println!("enter i32 integer");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("unable to read input");
    match input.trim().parse::<i32>() {
        Ok(n) => println!("your integer num {n}"),
        Err(e) => println!("about integer {e}"),
    }
}