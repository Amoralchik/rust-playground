// #[allow(unused_variables)] 

use std::ops::{Range, RangeInclusive}; // importing from std ops only Range and RangeInclusive
use std::mem::size_of_val;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() {
//     // i32 - 32 bit integer is also default 
//     // let x: i32 = 5;
    let _y: i32; // standard unused variable

    let mut s: i32 = 1; // can be mutated 
    s += 4;

    assert_eq!(s,5); // give error if not equal

//     {
//         let c: i32 = 10;
//         println!("The value of s: {} and c: {}", s,c)
//     }

//     // println!("The value of s: {} and c: {}", s,c) // return error because of local scope

    hello();
    modify();
    can();
    tuple();

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
    
        let _x: i32 = 42; // reassigning 
        // println!("{}", x); // 42
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
    
//         println!("Success!")
    }
    
}

fn two() {

    // println!("{}, {}", 128>>2, 1<<5); // 32, 32

    assert_eq!((1..5), Range{ start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    assert!(1u32 + 2u32 == 3u32);
    assert!(1i32 - 2i32 == -1i32);
    // assert!(1u8 - 2 == -1); // error

    assert!(3*50 == 150);
    assert!(9.6_f32 / 3.2_f32 == 3.0_f32);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0001
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // 0111
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0110 | shift + 6 = ^
    // println!("1 << 5 is {}", 1u32 << 5); // 32 
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x20

    let mut sum: i32 = 0;

    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    // for c in 'a'..='z' {
    //     println!("{}",c as u8)
    // }

    // assert!(0.1+0.2==0.3); // error because of 0.1+0.2 returns 0.30000000000000004 
    assert!(0.1_f32+0.2_f32==0.3 as f32); // so instead we can use f32

    let x: f64 = 1_000_000.000_1;
    // let y: f32 = 1.12;
    // let z: f64 = 1.01_f64;

    assert_eq!(type_of(&x), "f64");

    let number: i32 = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(number == 1597);

    // let k: u8 = 251_u8 + 8_u8; // return error
    // let r: i8 = i8::checked_add(251, 8).unwrap(); // also return error because of overflow
    // let k: u16 = 251_u16 + 8_u16;
    // let r: i16 = i16::checked_add(251, 8).unwrap();
    // println!("{},{}", k,r)

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let f: u16 = 38_u8 as u16;
    assert_eq!("u16", type_of(&f)); 

    fn type_of<T>(_:&T) -> String {
        format!("{}", std::any::type_name::<T>()) // wow
    }

    assign();    
    fn assign() {
    //     let x: i32 = 5;
    //     let mut y: u32 = 2;

    //     // y = x; // return error, because of different type

    //     let z: i32 = 10;
    }

    // just i funny reminder
    /*
        |-------------------------------------------------------|
        |  0   |   0  |   1   |   0  |   1   |   0  |   1   |  0  | > 42 in 8bit
        | 2(7) | 2(6) | 2(5) | 2(4) | 2(3) | 2(2) | 2(1) | 2(0) |
        | 128|   64   | 32   |  16  |  8  |   4  |  2   |   1  |
        |-----------------------------------------------------|
    
        so it's mean 
        i8 min: -128 and max 127
        but
        u8 min: 0 and max 255
    
        for float f32-f64 by IEEE-754 specification
    */
}

fn three() {
    // char 
    let c1: char = 'a'; // 4 bytes
    let c2: char = '本';// also 4 bytes
    // println!("{},{}", size_of_val(&c1),size_of_val(&c2));
    assert_eq!(size_of_val(&c1), 4);
    assert_eq!(size_of_val(&c2), 4);

    // let c1: &str = "a" // is not a char
    print_char(c1);

    fn print_char(_c: char) {
        // println!("{}", _c);
    }

    let t: bool = false;
    if !t {
        // println!("True")
    }

    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(t,f);

    let t: () = ();
    let _t: (i32,i32) = (2,3);
    assert_eq!(t,implicitly_ret_unit());

    assert!(size_of_val(&t) == 0);

    // char 4 bytes
    // bool 0 or 1 byte
    // Unit 0 bytes

    fn implicitly_ret_unit() -> () {
        // println!("return ()");
    }

}

fn four() {
    let x:u32 = 5u32;

    // statement?
    let _y:u32 = {
        // expressions
        let x_squared: u32 = x * x;
        let x_cube: u32 = x_squared * x;
        
        // this will be returned and assigned to "y"
        x_cube + x_squared + x
    };

    let _c: () = {
        // code with semicolons don't returns
        // 2 * _y;
    };

    let b:i32  = {
        let x: i32 = 1;
        x + 2
    };

    assert_eq!(b, 3);

    let a: i32 = sum(2,43);

    fn sum(q: i32, w:i32) -> i32 {
        // return q+w; // this also works
        q + w
    }

    assert_eq!(a, 45);
    
}

fn five() {
    // never_returns();

    // fn never_returns() -> ! {
    //     panic!("That's execute an ERROR");
    // }

    let q: () = un();

    fn un() {
        // unimplemented!()
        // todo!()
    }

    assert_eq!(q, ());

    // ownership 
    let x: String = String::from("hello");
    let _s: String = x.clone(); // so we need to use clone() because if we put just x here, the x variable itself will be removed
    // println!("{}, {}", x,_s);

    let x: Box<i32> = Box::new(1);
    let mut y: Box<i32> = Box::new(1);
    *y = 5;
    assert_eq!(*x, 1);

    // struct Person {
    //     name: String,
    //     age: Box<u8>,
    // }

    // let person: Person = Person {
    //     name: String::from("Alice"),
    //     age: Box::new(20)
    // };

    // let Person {name,ref age} = person;

    // println!("Person with name: {} at age of {}", name, age);

    let t: (String, String) = (String::from("hello"), String::from("world"));
    let _s: String = t.0;
    // println!("{:?}", t.1); // i cant use t and t.0, but still can use t.1

    let c:char = 'c';
    let r1: &char = &c; // create ref
    let ref r2 = *r1; // REF FOR REF LOL

    assert_eq!(r1,r2);
    assert_eq!(*r1,c); // * returns value

    assert_eq!(format!("{:p}", &c), format!("{:p}", r1));

    let m: &str = "stringc!"; // like String but ref
    let mut s: String = String::from(""); // we cant use &str here
    s.push_str("string");
    s.push('c'); // ? actually, why
    s += "!";

    assert_eq!(s, m);

    let s1: String = s.replace("stringc", "hello mom");
    assert_eq!(s1, "hello mom!");

    let s3: String = s1 + m;
    assert_eq!(s3, "hello mom!stringc!");

    // also need mention, 0..1 weird and mean's more like 0..0, so if we used 0..5 it's means give as items from 0 > 5 not 0 >= 5
    let h: &str = &s3[0..1]; // get first char in s3
    assert_eq!("h", h);
}
