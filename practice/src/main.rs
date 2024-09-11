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
}
