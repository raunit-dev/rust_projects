use std::fs;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct Counter {
    number: u8,
}

impl Counter {
    fn increment(&mut self) {
        self.number = self.number.saturating_add(1);
    }

    fn decrement(&mut self) {
        self.number = self.number.saturating_sub(1);
    }
}

fn main() {
    let mut c = Counter { number: 0 };

    // Serialize and write to file
    let mut buffer: Vec<u8> = Vec::new();
    c.serialize(&mut buffer).unwrap();
    println!("{:?}", buffer);
    let write_result = fs::write("no.txt", &buffer);
    match write_result {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(_) => println!("Error"),
    }

    // Read from file and deserialize
    let c_read = fs::read("no.txt");
    match c_read {
        Ok(contents) => {
            let deserialized = Counter::try_from_slice(&contents).unwrap();
            println!("File contents: {:?}", deserialized);
        }
        Err(_) => println!("Error while reading the file"),
    }

    // Example usage of increment and decrement
    c.increment();
    println!("After increment: {:?}", c);
    c.decrement();
    println!("After decrement: {:?}", c);
}