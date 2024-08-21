fn main() {
    #[allow(unused)]
    // let reference_to_nothing = dangle();
    let reference = no_dangle();
}

// fn dangle() -> &String{
//     let s = String::from("hello");

//     &s//the referene is live longer than the refrred data.
// }

fn no_dangle() ->String{
    let s: String = String::from("hello");

    s//Use s: s give the value ownership to return value. s is dropped.
}