enum NumberOrNothing {
    Number(i32),
    Nothing
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is <nothing>."),
            Number(n) => println!("The number is {}.", n)
        }
    }
}

use self::NumberOrNothing::{Number, Nothing};

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }

    let mut min = Nothing;

    for e in vec {
        min = Number(match min {
            Nothing => e,
            Number(n) => min_i32(n, e)
        });
    }

    min
}

fn read_vec() -> Vec<i32> {
    vec![4, 3, -2, 1, 10, 8]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
