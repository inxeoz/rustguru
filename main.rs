mod num_by_prime;

fn main() {
    let n = input().parse::<usize>().expect("enter positive num");
    num_by_prime::val(n);

}

fn input() -> String{
    use std::io::stdin;
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}