pub fn year(y:u64){
    if (y % 4 ==0 && y % 100!= 0 )|| (y % 400 == 0 ){
        println!("leap year {y}");
    }else{
        println!("not leap year")
    }
}