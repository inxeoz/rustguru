pub fn run(input: String) {
    match input.trim().parse::<i32>() {
        Ok(n) => println!("your integer num {n}"),
        Err(e) => println!("about integer {e}"),
    }
}