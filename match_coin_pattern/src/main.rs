#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    //...
}

#[allow(dead_code)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main(){
    let alabama = Coin::Quarter(UsState::Alabama);
    let alaska = Coin::Quarter(UsState::Alaska);

    println!("{}",value_in_cents(alabama));
    println!("{}",value_in_cents(alaska));
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}