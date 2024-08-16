use std::io;

//The classic invalid array element access
fn main() {
    let a = [1,2,3,4,5];

    println!("Please enter an array index:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");//enter number 5

    let index:usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
