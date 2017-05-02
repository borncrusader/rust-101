use std::io::prelude::*;
use std::io;

fn read_vec() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::<i32>::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with EOF");

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<i32>() {
            Ok(num) => {
                vec.push(num)
            },
            Err(_) => {
                println!("That's not a number!");
            },
        }
    }

    vec
}

fn main() {
    let vec = read_vec();

    println!("{:?}", vec);
}
