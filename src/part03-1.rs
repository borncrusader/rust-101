trait Print {
    fn print(self);
}

impl Print for i32 {
    fn print(self) {
        println!("{}", self)
    }
}

fn main() {
    let x: i32 = 5;

    x.print();
}
