
fn main(){
    let five = Some(5);
    let size = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}",five,size,none);
}
fn plus_one(x: Option<i32>) -> Option<i32>{
	match x {
		None => None,
		Some(i) => Some(i+1),
	}
}