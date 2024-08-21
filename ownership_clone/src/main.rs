fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone();//clone
    println!("{}",s1.len());//Don't cause errors
    println!("{}",s2.len());
}