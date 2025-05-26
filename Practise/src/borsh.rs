use std::vec;

use borsh::{BorshDeserialize,BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize,Debug,PartialEq)]
struct Developer {
    id: u64,
    name: String,
    gfs: Vec<u32>
}

fn main() {
    let s = Developer {
        id: 19,
        name: "raunit".into(),
        gfs: vec![0,0,0]
    };

    let mut buffer: Vec<u8> = Vec::new();
    s.serialize(&mut buffer).unwrap();
    println!("{:?}",buffer);
    let deserialized = Developer::try_from_slice(&mut buffer).unwrap();

    assert_eq!(s, deserialized);
    println!("Successfully serialized and deserialized: {:?}", deserialized);
}