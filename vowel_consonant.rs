pub fn vo_co(a:char){
    let c = a.clone().to_ascii_lowercase();
    if "aieou".contains(c){
        println!("{a} is vowel ");
    }else {
        println!("{a} is consonant");
    }
}