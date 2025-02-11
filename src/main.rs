// #[allow(unused_variables)] 

fn main() {
    // i32 - 32 bit integer
    // let x: i32 = 5;
    let _y: i32; // standard unused variable

    let mut s: i32 = 1; // can be mutated 
    s += 4;

    assert_eq!(s,5); // give error if not equal

    {
        let c: i32 = 10;
        println!("The value of s: {} and c: {}", s,c)
    }

    // println!("The value of s: {} and c: {}", s,c) // return error because of local scope
    hello();
    modify();
    can();
    tuple();
}

fn hello() {
    let x:&str = "hello";
    println!("{}, world!", x)
}

fn modify() {
    let x: i32 = 5;
    {
        let x: i32 = 10;
        assert_eq!(x,10) // x = 10, true
    }
    assert_eq!(x,5); // main X still have value of 5, true

    let x: i32 = 42; // reassigning 
    println!("{}", x); // 42
}

fn can() {
    let mut _x: i32 = 1;
    _x = 7;
    
    let _x: i32 = _x;
    // x += 3; // give error, because of missing "mut" in reassigning 

    let _y: i32 = 4;
    let _y: &str = "You can assign to other type";
}

fn tuple() {
    let (mut x,y) = (1,2);
    x += 2;

    assert_eq!(x,3);
    assert_eq!(y,2);

    let (x,y); // mut?
    (x,..) = (3,4);
    [..,y] = [1,2];
    assert_eq!([x,y], [3,2]);

    println!("Success!")
}
