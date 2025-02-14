// #[allow(unused_variables)] 

use std::ops::{Range, RangeInclusive}; // importing from std ops only Range and RangeInclusive
use std::mem::size_of_val;

fn main() {
    one();
    two();
    three();
    four();
    five();
    six();
    seven();
    eight();
    nine();
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

fn six() {
    let list: [i32; 100] = [1; 100]; // create array with 100 time of 1
    assert_eq!(list[0], 1);
    assert_eq!(list.len(), 100);

    let slice: &[i32] = &list[0..3];
    assert_eq!(slice, [1,1,1]);

    let arr: [char; 3] = ['a','b','c'];
    let slice: &[char] = &arr[..2];
    
    assert_eq!(std::mem::size_of_val(&slice), 16);
    assert_eq!(std::mem::size_of_val(&arr), 12); // 4+4+4

    let arr: [i32;5] = [1,2,3,4,5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2,3,4]);

    let s: String = String::from("value");
    let slice:&str = &s[0..2];
    let slice2: &str = &s[..2];
    assert_eq!(slice,slice2);

    let mut s: String = String::from("value");
    let word: &str = &s[..1];
    assert_eq!(word,"v");

    s.clear();

}

fn seven() {
    let (x,y,z);
    (y,z,x) = (1,2,3);
    assert_eq!(x,3);
    assert_eq!(y,1);
    assert_eq!(z,2);

    let (x,y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    fn sum_multiply(nums: (i32,i32)) -> (i32,i32) {
        (nums.0 + nums.1, nums.0 * nums.1)
    }

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }

    let age: u8 = 24;
    let p: Person = Person {
        name: String::from("Amo"),
        age,
        hobby: String::from("coding"),
    };

    assert_eq!(p.age, 24);
    assert_eq!(p.name, "Amo");
    assert_eq!(p.hobby, "coding");
    
    struct Point(i32,i32,i32);
    let c: Point = Point(54, 42, 23);
    check(c);
    fn check(p: Point) {
        let Point (x, _, c) = p;
        assert_eq!(x, 54);
        assert_eq!(p.1, 42);
        assert_eq!(c,23);
    }

    let mut p: Person = Person {
        name: String::from("Amo"),
        age,
        hobby: String::from("coding"),
    };

    p.age = 30;
    p.name = String::from("value");

    assert_eq!(p.age, 30);
    assert_eq!(p.name, "value");
    assert_eq!(p.hobby, "coding");
    
    fn build_person(name: String, age: u8) -> Person {
        Person {
            name,
            age,
            hobby: String::from("coding"),
        }
    }

    let o: Person = build_person(String::from("Bo"), 54);

    assert_eq!(o.age, 54);
    assert_eq!(o.name, "Bo");
    assert_eq!(o.hobby, "coding");

    fn clone_with_diff(u: Person, name: String) -> Person {
        Person {
            name,
            ..u
        }
    }

    let o2: Person = clone_with_diff(o,String::from("Tom"),);

    assert_eq!(o2.age, 54);
    assert_eq!(o2.name, "Tom");
    assert_eq!(o2.hobby, "coding");

    let name: String = o2.name.clone();
    assert_eq!(o2.name, name);

    // println!("{:?}", o2);
    // dbg!(&o2);
}

fn eight() {
    enum Number {
        Zero = 0,
        One,
        Two,
    }

    enum Number2 {
        Zero,
        One,
        Two,    
    }

    assert_eq!(Number::Zero  as u8,Number2::Zero  as u8);
    assert_eq!(Number::One  as u8, Number2::One  as u8);
    assert_eq!(Number::Two  as u8, Number2::Two  as u8);
    assert_eq!(Number::One as u8, 1);

    // println!("{}", Number::One as u8);
    
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32,i32,i32),
    }

    let msg1: Message = Message::Move { x: 2, y: 2 };
    let msg2: Message = Message::Write(String::from("value"));

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::ChangeColor(255, 255, 0)
    ];

    if let Message::Move { x: a, y: b } = msg1 {
        assert_eq!(a,b);
    } else {
        panic!("PANIC");
    }

    if let Message::Write(x) = msg2 {
        assert_eq!(x, "value");
    } else {
        panic!("PANIC");
    }

    if let Message::ChangeColor(a,b,c) = msgs[2] {
        assert_eq!(a, 255);
        assert_eq!(b, 255);
        assert_eq!(c, 0);
    } else {
        panic!("PANIC");
    }

    // for msg in msgs {
    //     println!("{:?}", msg);
    // }

    // enum Option<T> {
    //     None,
    //     Some(T), // any type
    // }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let _none: Option<i32> = plus_one(None);

    if let Some(n) = six {
        assert_eq!(n, 6);
    }

}

fn nine() {
    let a: [i32; 4] = [4,3,2,1];
    for (i,v) in a.iter().enumerate() {
        if i == 0 {
            assert!(i == 0 && *v == 4);
        } else if i == 1 {        
            assert!(i == 1 && *v == 3);
        } else if i == 2 {
            assert!(i == 2 && *v == 2);
        } else {
            assert!(i == 3 && *v == 1);
        }
    }

    let mut n: i32 = 1;
    while n < 10 {
        if n % 15 == 0 {
            // println!("fizzbuzz")
        } else if n % 3 == 0 {
            // println!("fizz")
        } else if n % 5 == 0 {
            // println!("buzz")
        } else {
            // println!("{}",n)
        }
        n += 1;
    }

    let mut n: i32 = 1;
    for i in 0..=100 {
        if n != 70 {
            n = i;
            continue;
        }
        if n >= 66 {
            break;
        }
    }
    assert_eq!(n, 70);

    let mut n: i32 = 1;
    let result: i32 = loop {
        n += 1;
        if n == 2 {
            assert_eq!(n, 2);
            continue;
        }
        if n == 5 {
            break n * 2; // the only way to end the Loop
        }
    };

    assert_eq!(n, 5);
    assert_eq!(result, 10);

    let mut count: i32 = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }
        count += 5;
        'inner2: loop {
            if count >= 30 {
                break 'inner2;
            }
            continue 'outer;
        }
        break 'outer;
    }
    assert_eq!(count,30);
    
    let a: [char; 7] = ['a','E','Z','0','x','9', 'Y'];
    for c in a {
        assert!(matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }

    let o: Option<i32> = Some(7);
    if let Some(i) = o {
        assert_eq!(i,7);
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle {
                width, height
            }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn change_width(&mut self, value:u32) {
            self.width = value
        }
    }

    let mut rect: Rectangle = Rectangle::new(20, 50);
    // println!("Rectangle: {:?}", rect);
    assert_eq!(rect.height, 50);
    assert_eq!(rect.width, 20);
    assert_eq!(rect.area(), 1000);

    rect.change_width(30);
    assert_eq!(rect.area(), 1500);

}