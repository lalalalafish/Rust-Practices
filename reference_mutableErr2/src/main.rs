fn main() {
    let mut s = String::from("hello");

    let r1 = &s;//no problem
    let r2 = &s;//no problem
    let r3 = &mut s;//ERROR

    println!("{r1} {r2} {r3}");
}
