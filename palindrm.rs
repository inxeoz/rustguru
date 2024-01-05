pub fn pln(s: &str){
    let binding = s.chars().rev().collect::<String>();
    let sr = binding.as_str();
    if sr.eq_ignore_ascii_case(s){
        println!("{s} pln {sr}");
    }else{
        println!("not pln");
    }
}