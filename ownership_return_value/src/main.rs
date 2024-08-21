fn main(){
    let s1 = gives_ownership();//gives_ownership moves its return value into s1
    println!("{}",s1);
    let s2 = String::from("hello");
    println!("{}",s2);

    let s3 = takes_and_gives_back(s2);//s2 moves into takes_and_gives_back
                                    //and then takes_and_gives_back moves its return value into s3
    println!("{}",s3);
}//pop s3(drop), pop s2, pop s1)(drop)

fn gives_ownership() -> String{
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}