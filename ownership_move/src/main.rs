fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;//move
    println!("{}",s1.len());
    println!("{s2}");
}
