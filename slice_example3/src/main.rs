fn main() {
    let mut s = String::from("hello world");

    let slice = first_word(&s);
    s.clear();
    println!("slice: {slice}");//Slice is sync with the data s

}

fn first_word(s: &String) -> &str{
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate(){

		if item == b' '{
			return &s[0..i];
		}
	}
	&s[..]
}
