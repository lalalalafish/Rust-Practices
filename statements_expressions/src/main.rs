fn main() {
    let t = plus_two(64);//ERROR
    println!("The value of t is {t}");

    // let x = 5;//Assignment is a statement
    // let y = (let z = 12);//ERROR

    //Expression: scope block with{}
    let y = {
        let x = 3;
        x+1
    };
    println!("The value of y is {y}");

    //Expression: function calling 
    let z = another_function(32);
    println!("The value of z is {z}");

    //Expression: macro calling
    let str= format!("Hello! {}",123);
    println!("The value of str is {str}");

}

//Statement:: function definition
fn another_function(x:i32)->i32{
    println!("The value of x is {x}");
    x+2
}

fn plus_two(x:i32) ->i32{
    x+2;//ERROR
}