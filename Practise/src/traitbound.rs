// trait Shape {
//     fn area(&self) -> u32;
// }

// struct Rect {
//     height: u32,
//     width: u32,
// }

// impl Shape for Rect {
//     fn area(&self) -> u32 {
//         return self.height*self.width
//     }
// }

// fn main() {
//     let rect = Rect { height: 10, width: 5 };
//     println!("Hello World!");
//     println!("Area: {}", get_area(rect));
// }

// fn get_area<T: Shape>(s: T) -> u32 {
//     return s.area();
// }


#[derive(Debug)]
struct User {
    username: String
}
fn main () {
    let u = User {
        username: String::from("harkirat")
    };
    println!("{:?}",u);
}