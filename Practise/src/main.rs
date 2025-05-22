use std::fs;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Square {
    length: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.length * self.length
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.length
    }
}

struct Rect {
    height: f32,
    width: f32,
}

impl Rect {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }

    fn print_something(_a: u32) {
        println!("Static Function");
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Shapes {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

struct DirectionType {
    direction: Direction,
}

fn calculate_area(s: Shapes) -> f32 {
    match s {
        Shapes::Square(side) => side * side,
        Shapes::Circle(radius) => std::f32::consts::PI * radius * radius,
        Shapes::Rectangle(width, height) => width * height,
    }
}

fn main() {
    let r = Rect {
        width: 10.0,
        height: 10.0,
    };

    let square = Square { length: 2.0 };

    let dir = DirectionType {
        direction: Direction::East,
    };

    let answer = match dir.direction {
        Direction::East => true,
        Direction::West => true,
        Direction::North => true,
        Direction::South => true,
    };

    println!("{}", answer);

    let shape = Shapes::Square(10.0);
    let shape_circle = Shapes::Circle(10.0);
    let shape_rect = Shapes::Rectangle(10.0, 10.0);

    println!("{}", calculate_area(shape));
    println!("{}", calculate_area(shape_circle));
    println!("{}", calculate_area(shape_rect));

    let my_direction = Direction::North;
    println!("{:?}", my_direction);

    let day2 = "Day2";
    println!("{}", day2);

    println!("{}, {}", r.width, r.height);
    println!("{}", r.area());
    println!("{}", r.perimeter());
    println!("{}", square.length);
    println!("{}", square.area());
    println!("{}", square.perimeter());
    Rect::print_something(10);


    let contents =  fs::read_to_string("a.txt");

    match contents {
        Ok(contents ) => println!("{}",contents),
        Err(_) => println!("Error while reading the file")
    }

    let write_result = fs::write("a.txt", "Hey i am writing into a file");

    match write_result {
        Ok(_) => {
            println!("Successfully wrote to the file.");
            // Read back the contents to print what was written
            match fs::read_to_string("a.txt") {
                Ok(contents) => println!("File contents: {}", contents),
                Err(_) => println!("Error while reading the file"),
            }
        }
        Err(_) => println!("Error while writing the file"),
    }
}
