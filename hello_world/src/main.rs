mod maths;
mod folder;

struct User {
    name: String,
    age: u32,
    address: String,
    salary: Option<i16>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!("Hello, world!");

    let mut my_string = String::from("Hello, world!");

    let new_string = update_string(&mut my_string);
    print!("New string returned after borrowing is as follows: {}", new_string);
    println!("{}", my_string);
    let user: User = User {
        name: String::from("John"),
        age: 25,
        address: String::from("New York"),
        salary: Some(1000),
    };
    println!("User name is: {}", user.name);
    println!("User age is: {}", user.age);
    println!("User address is: {}", user.address);
    println!("User salary is: {:?}", user.salary);

    let direction = Direction::Up;
    let right_direction = Direction::Right;
    let left_direction = Direction::Left;
    let down_direction = Direction::Down;
    see_direction(right_direction);
    see_direction(left_direction);
    see_direction(down_direction);
    see_direction(direction);

    let result = maths::add(10, 20);
    println!("Result of addition is: {}", result);


    let perimeter = folder::perimeter(10.0); 
    println!("Perimeter of circle is: {}", perimeter);
}

fn update_string(s: &mut String) -> &mut String {
    s.push_str("!");
    return s;
}

fn see_direction(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}
