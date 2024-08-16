fn main() {
    let tup:(i32,f64,char,bool) = (12,32.5,'Z',true);//Declaration
    let (x,y,z,t) = tup;//Access1
    println!("({}, {}, {}, {})",x,y,z,t);

    let a = tup.0;//Access2
    let b = tup.1;
    let c = tup.2;
    let d = tup.3;
    println!("({}, {}, {}, {})",d,c,b,a);
}
