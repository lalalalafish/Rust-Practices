fn main() {
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);

    println!("s2: {}\nlen: {}",s2,len);
}

fn calculate_length(s:String) -> (String, usize){
    let length = s.len();
    (s,length)
}
