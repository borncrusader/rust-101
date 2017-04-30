#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]

enum SomethingOrNothing<T> {
    Something(T),
    Nothing
}

use SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<i32>;

impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o {
            Some(t) => Something(t),
            None => Nothing
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            Something(t) => Some(t),
            Nothing => None
        }
    }
}

fn call_constructor(x: i32) -> NumberOrNothing {
    SomethingOrNothing::new(Some(x))
}

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

impl Minimum for i32 {
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

fn read_vec() -> Vec<i32> {
    vec![3, 4, 2, -1, 10, 20]
}

fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
