#![allow(dead_code)]

enum SomethingOrNothing<T> {
    Something(T),
    Nothing
}

type NumberOrNothing = SomethingOrNothing<f32>;

use SomethingOrNothing::*;

trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}

fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;

    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => {
                e.min(n)
            }
        });
    }

    min
}

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Something(n) => println!("The number is {}.", n),
            Nothing => println!("The number is <nothing>.")
        };
    }
}

fn read_vec() -> Vec<f32> {
    vec![3.0, 4.0, 2.2, -1.2, 10.0, 20.1]
}

fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
