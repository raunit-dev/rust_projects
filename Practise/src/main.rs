// use std::ops::Add;
// fn main() {
//     println!("{}", sum(1,2));
// }

// fn sum<T: Add<Output = T>>(a: T,b: T) -> T {
//     return a + b;
// }


trait Coding {
    fn code(&self);
}

struct Developer;
struct Designer;

impl Coding for Developer {
    fn code(&self) {
        println!("Developer is writing Rust Code");
    }
}
fn sign_up<T: Coding>(person: T) {
    println!("Signing up...");
    person.code();
}

fn main() {
    let dev = Developer;
    sign_up(dev);

    // let des = Designer;
    // sign_up(des);
}