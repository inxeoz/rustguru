pub fn root(a:f64, b:f64, c:f64){
    let det:f64 = b*b - 4f64 * a*c;
    let mut rt1: f64;
    let mut rt2: f64;
    if det > 0f64 {
        rt1  = (-b + det.sqrt() )  / (2f64 * a);
        rt2 = (-b - det.sqrt()) / (2f64 * a);
        println!("rt1 = {rt1}, rt2 = {rt2}");

    }else if (det == 0f64){
        rt1 = -b / (2f64 *a);
        rt2= rt1;
        println!("rt1 = rt2 = {rt1}");

    }else{
        let real  = -b / (2f64 *a);
        let img = (-det).sqrt() / (2f64 * a);
        println!("rt1 = {} + {}i" , format!("{:.2}", real), format!("{:.2}", img));
        println!("rt1 = {} - {}i" , format!("{:.2}", real), format!("{:.2}", img));

    }
}