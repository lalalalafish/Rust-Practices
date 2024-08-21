fn main() {
    let s = String::from("hello, world");

    let first_word1 = &s[0..5];//start_index .. end_index+1
    let first_word2 = &s[..5];//0 can be omitted

    let len = s.len();
    let second_word1 = &s[7..len];//variable can be used as index
    let second_word2 = &s[7..];//len can be omitted

    let whole1 = &s[0..len];
    let whole2 = &s[..];//0 and len can be omitted at the same time

    println!("s: {}\nlen: {}",s,len);
    println!("First word: {}/{}",first_word1,first_word2);
    println!("Second word: {}/{}",second_word1,second_word2);
    println!("Whole string: {}/{}",whole1,whole2);


    
}
