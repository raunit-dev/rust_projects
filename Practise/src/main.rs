trait Shape {
    fn area(&self) -> f64; 
    fn perimeter(&self) -> f64;
}

struct Square {
   length: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return  self.length  * self.length;
    }

    fn perimeter(&self) -> f64 {
        return 4.0 * self.length;
    }
}

struct Rect {
    height: f32,
    width: f32
}

impl Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> f32 {
        return 2.0 * (self.width + self.height);
    }

    fn print_something(_a: u32) {
        println!("Static Function");
    }
}

//match.enums,struct damnnnnn

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct DirectionType {
    direction : Direction
}



fn main () {
    let r = Rect {
        width: 10.0,
        height: 10.0
    };

    let Square = Square {
        length: 2.0
    };

    let dir = DirectionType {
        direction: Direction::East
    };
 
    let answer = match dir.direction {
        Direction::East => {
            true
        }
        ,
        Direction::West => {
            true
        },
        Direction::North => {
            true
        },
        Direction::South => {
            true
        }
    }; 

    println!("{}",answer);

    

    let my_direction = Direction::North;
    println!("{:?}", my_direction);

    println!("{}, {}", r.width, r.height);
    println!("{}", r.area());
    println!("{}", r.perimeter());
    println!("{}", Square.length);
    println!("{}", Square.area());
    println!("{}", Square.perimeter());
    Rect::print_something(10);
}