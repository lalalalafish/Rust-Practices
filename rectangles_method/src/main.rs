#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    //method
    fn area(&self) -> u32{
        self.width * self.height
    }
    //getters
    fn width(&self) -> u32{
        self.width
    }

    fn is_hold(&self, other: &Rectangle) -> bool{
        self.width * self.height >= other.width * other.height
    }

    fn square(size: u32) -> Self{
        Self{
            width: size,
            height: size,
        }
    }
}

fn main(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 40,
        height: 40,
    };

    let sq = Rectangle::square(24);
    
    println!("The area of rect1: {}",rect1.area());
    println!("The width of rect2: {}",rect2.width());
    println!("rect1 can hold rect2: {}",rect1.is_hold(&rect2));
    println!("The square: width:{}, height: {}",sq.width,sq.height);
}