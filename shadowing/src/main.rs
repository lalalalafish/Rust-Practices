fn main() {
    let x =5;
    let x = x+1;//Shadowing
    {
        let x = x * 2;//shadowing
        println!("The value of x in the inner scope is {x}");
        //this shadowing variable is only valid in the inner scope
    }
    println!("The value of x is {x}");

    //Shadowing VS Mutable variable
    let mut spaces = "    ";
    let spaces = spaces.len();//Shadowing: the variable with different type can also shadow the previous variable.

    let mut spaces = "   ";
    spaces = spaces.len();//ERROR: cannot change the type of mut variable
}
