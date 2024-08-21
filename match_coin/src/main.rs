enum Coin{
	Penny,//1¢
	Nickel,//5¢
	Dime,//10¢
	Quarter,//25¢
}

fn main(){
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;

    println!("p: {}",value_in_cents(p));
    println!("n: {}",value_in_cents(n));
    println!("d: {}",value_in_cents(d));
    println!("q: {}",value_in_cents(q));
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin{
		Coin::Penny => {//A arm with statements
			println!("Lucky penny!");
			1
		}
		Coin::Nickel => 5,// A arm with an expression.
		Coin::Dime => 10,
		Coin::Quarter => 25,
	}
}