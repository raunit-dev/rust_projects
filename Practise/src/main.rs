use std::ops::Add;
use std::ops::Mul;
// fn main() {
//     println!("{}", sum(1,2));
// }


// fn sum<T: Add<Output = T>>(a: T,b: T) -> T {
//     return a + b;
// }


// trait Coding {
//     fn code(&self);
// }

// struct Developer;
// struct Designer;

// impl Coding for Developer {
//     fn code(&self) {
//         println!("Developer is writing Rust Code");
//     }
// }
// fn sign_up<T: Coding>(person: T) {
//     println!("Signing up...");
//     person.code();
// }

// fn main() {
//     let dev = Developer;
//     sign_up(dev);

//     // let des = Designer;
//     // sign_up(des);
// }

// use std::fmt::Display;

// struct User {
//     username: String
// }

// fn main() {
//     let u1 = User {
//         username: String::from("raunit")
//     };
 
//     let u2 = User {
//         username: String::from("jaiswal")
//     };
//     display_elements(1,2);
//     display_elements(String::from("Raunit"), String::from("Jaiswal"));
//     display_elements(u1, u2);
// }

// fn display_elements<T: Display>(a: T, b: T) {
//     println!("{}",a);
//     println!("{}",b);
// }

use std::fmt::Display;



struct Rect<T> {
    width: T,
    height: T,
}

impl<T: Mul<Output = T> + Copy> Rect<T> 
// where
//     T: Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 10,
    };
    
    println!("{}", r.area());
}