fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();//set s to ""
    println!("{}",word);//word is not sync with s
}

fn first_word(s: &String) -> usize{
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate(){
	//iter method：returns each element in a collection
	//enumerate method: wraps the result of `iter` and returns a uple:
	//i: the index
	//&item: the reference to the element
		if item == b' '{
			return i;
		}
	}
	s.len()
}
