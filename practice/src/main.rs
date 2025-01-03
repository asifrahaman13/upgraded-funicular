use std::vec;

struct Address {
    x: i32,
    name: String,
}

fn main() {
    let mut v = vec![1, 2, 3];
    square_vector(&mut v);
    print!("{:?}", v);
    for item in 1..34 {
        println!("The item is: {}", item);
    }
    for (index, item) in (5..23).enumerate() {
        println!("The item index is: {} and the value is: {}", index, item);
    }
    let name = String::from("Kailash");
    let employee = Address { x: 23, name: name };
    println!("The string here is: {} and the value of the x is: {}", employee.name, employee.x);
}

fn square_vector(v: &mut Vec<i32>) {
    for item in v {
        *item = *item * *item;
    }
}
