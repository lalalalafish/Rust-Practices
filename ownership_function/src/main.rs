fn main(){
    let s = String::from("hello");//s push into stack

    takes_ownership(s);//s's value moves into the funtion
                                    //And so is no longer valid here
    
    let x = 5;//x push into stack

    makes_copy(x);//x would move into the function
                                //x is still can be used afterward.
    
}//pop x and then pop s, but the value of s was moved, nothing special happens on s.

fn takes_ownership(some_string: String){//some_string push into stack
    println!("{}",some_string);
}//pop some_string and call `drop`. The backing memory is freed

fn makes_copy(some_integer: i32){//some_integer push into stack
    println!("{}",some_integer);
}//pop some_integer