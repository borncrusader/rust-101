fn vec_sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;

    for e in vec {
        sum = sum + e;
    }

    sum
}

fn read_vec() -> Vec<i32> {
    vec![4, 3, -2, 1, 10, 8]
}

pub fn main() {
    let vec = read_vec();
    let sum = vec_sum(vec);
    println!("The sum is {}.", sum);
}
