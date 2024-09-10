struct Employee{
    id: i32,
    name: String, 
    address: String
}

// Another equivalent of the enum in rust is the tuple. 
// It is a kinda hybrid between the tuple and the struct 

struct Colors(i32, String, i8);

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


    let my_employee=Employee{
        id: 12,
        name: String::from("Harris"),
        address: String::from("33 Harrison road.")
    };

    println!("The details of the person: {} {} {}", my_employee.id, my_employee.name, my_employee.address);


    let my_colors=Colors(12, String::from("green"), 3);

    println!("The color is: {}", my_colors.2)
}
