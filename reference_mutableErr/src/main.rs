fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("r1: {r1}\nr2: {r2}");
}
