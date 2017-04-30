fn vec_print(vec: Vec<i32>) {
    print!("[");

    for e in vec {
        print!("{}, ", e);
    }

    println!("]");
}

fn read_vec() -> Vec<i32> {
    vec![4, 3, -2, 1, 10, 8]
}

pub fn main() {
    let vec = read_vec();
    vec_print(vec)
}
