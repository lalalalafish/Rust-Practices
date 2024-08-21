fn main() {
    let mut s = String::from("hello");

    let r1 = &s;//immutable reference
    let r2 = &s;//immutable reference
    println!("{r1} and {r2}");//r1 and r2 go out of the scope

    {
        #[allow(unused)]
        let r3 = &mut s;//mutable reference
    }//r3 go out of the scope

    let r4 = &mut s;//mutable reference
    println!("{r4}");//r4 go out of the scope
}
