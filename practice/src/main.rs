use std::vec;

fn main() {
    let mut v = vec![1, 2, 3];
    square_vector(&mut v);
    print!("{:?}", v);
}

fn square_vector(v: &mut Vec<i32>) {
    for item in v {
        *item = *item * *item;
    }
}
