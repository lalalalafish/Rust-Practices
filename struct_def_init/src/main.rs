//Struct definition
struct User{
    active: bool,
    email: String,
    name: String,
    sign_in_count:i32,
}

//Tuple struct definition
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

//Unit-like struct
struct Unit;

fn main() {
    //Instantiation
    let mut user1 = User{
        active: true,
        email: String::from("first@example.com"),
        name: String::from("Peter"),
        sign_in_count: 1,
    };

    //Access and change
    user1.sign_in_count = 12;

    let user2 = User{
        email: String::from("second@example.com"),
        name: String::from("Mike"),
        ..user1//copy
    };

    println!("user1:");
    println!("active: {}",user1.active);
    println!("email: {}",user1.email);
    println!("name: {}",user1.name);
    println!("sign_in_count: {}",user1.sign_in_count);

    println!("user2:");
    println!("active: {}",user2.active);
    println!("email: {}",user2.email);
    println!("name: {}",user2.name);
    println!("sign_in_count: {}",user2.sign_in_count);

    let user3 = User{..user1};//move 

    println!("user3:");
    println!("active: {}",user3.active);
    println!("email: {}",user3.email);
    println!("name: {}",user3.name);
    println!("sign_in_count: {}",user3.sign_in_count);

    //Tuple struct
    let mut color = Color(12,23,21);
    let point = Point(1,2,3);
    //Access and change
    color.0 = 33;
    println!("color: ({}, {}, {})",color.0,color.1,color.2);
    println!("point: ({}, {}, {})",point.0,point.1,point.2);

    //Unit struct
    #[allow(unused)]
    let unit = Unit;

}
