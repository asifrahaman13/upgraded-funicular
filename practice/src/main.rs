use std::ops::Add;

struct Employee {
    id: i32,
    name: String,
    address: String,
    salary: i32,
}

// Another equivalent of the enum in rust is the tuple.
// It is a kinda hybrid between the tuple and the struct

struct Colors(i32, String, i8);

// method syntax in rust is similar to the interfact and the struct relationships.
// The advantage is that we first define the trait and later we implement it. As a result we are sure
// of the behaviour of the method.
impl Employee {
    fn print_name(&self) -> String {
        println!("This is a sample print");
        return self.name.clone();
    }

    fn increment_salary(&self) -> i32 {
        return self.salary * 2;
    }
}

struct Address<Type> {
    address_name: Type,
    address_pin: Type,
}

struct Point {
    x: i32,
    y: i32,
}

// Traits looks similar to that of the struct and the implementation.

trait PrintSomething {
    fn print_something(&self);
}

impl PrintSomething for Point {
    fn print_something(&self) {
        println!("The address is: {} and the pin is: {}", self.x, self.y);
    }
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    // Static function. No need to create an instance of the struct to call this function.
    fn debug() -> String {
        return String::from("Hey this is degugging.");
    }
}
#[derive(Debug)]
enum Shape {
    Circle(u128),
    Rectangle(u128, u128),
    Square(u128),
}

// If we want to implemenet a trait and it can also return a null value then we should return option.
fn find_first_a(s: String) -> Option<i32> {
    let mut idx = 0;
    for (index, chat) in s.chars().enumerate() {
        if chat == 'a' {
            idx = index;
            break;
        }
    }
    return Some(idx as i32);
}

fn main() {
    println!("Hello, world!");
    let (x, y) = (1, 2);
    println!("The numbers are: {} {}", x, y);
    let shadow = 23;
    {
        let shadow: i32 = 434;
        println!("The value of shadow is: {}", shadow);
    }
    println!("The value of shadow now is: {}", shadow);
    for (index, item) in (1..10).enumerate() {
        println!("The index :{} and the value is: {}", index, item);
    }
    let mut new_vectors = Vec::new();
    new_vectors.push(23);
    new_vectors.push(34);
    println!("The new vector is: {:?}", new_vectors);

    let my_employee = Employee {
        id: 12,
        name: String::from("Harris"),
        address: String::from("33 Harrison road."),
        salary: 150000,
    };

    println!(
        "The details of the person: {} {} {}",
        my_employee.id,
        my_employee.name,
        my_employee.address
    );

    let my_colors = Colors(12, String::from("green"), 3);
    println!("The color is: {}", my_colors.2);
    println!("The number color is: {}", my_colors.1);
    println!("The number color is: {}", my_colors.0);

    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Others"),
    }

    let return_value = my_employee.print_name();
    println!("The return value from the function is: {}", return_value);

    let incremented_salary = my_employee.increment_salary();
    println!("The incremented salary is: {}", incremented_salary);

    let my_address = Address { address_name: "Berhampore", address_pin: "234232" };
    println!(
        "The address details is: {} and pin is: {}",
        my_address.address_name,
        my_address.address_pin
    );

    let my_trait = Point { x: 23, y: 34 };

    my_trait.print_something();

    let my_rectange = Rect { width: 23, height: 34 };
    println!("The area of the rectangle is: {}", my_rectange.area());

    println!("The debug value is: {}", Rect::debug());

    let my_shape = Shape::Circle(5);
    println!("The shape is: {:?}", my_shape);

    let first_a = find_first_a(String::from("Harris"));

    match first_a {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value found"),
    }
}
