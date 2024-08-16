fn main() {
    //ERROR: need type annotation
    // let x = "42".parse();
    // println!("The value of x is {x:?}");

    let x:u8 = b'A';
    println!("The value of x is {x}");

    //char
    let c = 'Z';
    let z:char = '😻';
    println!("c: {c}\nz: {z}");
}
