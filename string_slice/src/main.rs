fn main() {
    let my_string = String::from("hello, world");

    //Slices of `String`
    let word = first_word(&my_string[0..6]);
    println!("{}",word);
    let word = first_word(&my_string[..]);
    println!("{}",word);
    //References to `String`
    let word = first_word(&my_string);
    println!("{}",word);
    //Slices of String literal
    let my_string_literal = "hello, world";
    let word = first_word(&my_string_literal[0..6]);
    println!("{}",word);
    let word = first_word(&my_string_literal[..]);
    println!("{}",word);
}

fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]
        }
    }
    &s[..]
}

