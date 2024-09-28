use std::fs;
use chrono::{ Local, Utc };

mod maths {
    pub mod mensuration;
    pub mod addition;
}

mod string_slices {
    pub mod string_slices;
}

mod traits{
    pub mod traits;
}

mod lifetimes{
    pub mod lifetimes;
}

use maths::mensuration::area;
use maths::addition::addition;
use traits::traits::traits;
use lifetimes::lifetimes::lifetimes;

use string_slices::string_slices::string_slice;

fn main() {
    let file_text = read_file_content("src/hello.txt");
    println!("The file text is: {:?}", file_text);

    // The function will take care of the error handling and will return the data if the file is read successfully.
    let local_time = Local::now();
    println!("Local time is: {}", local_time);

    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("The value is: {}", val);
    }
    let now = Utc::now();
    println!("UTC time is: {}", now);
    let my_string = String::from("some value");

    let copied_string = my_string; // Here the copied string is the owner and the my_string goes out of the scope and no longer the owner of the value.
    println!("The value of the copied string is: {}", copied_string);
    do_something(&copied_string);

    let mut mutable_variable = String::from("Hello I am first");
    change_something(&mut mutable_variable);
    println!("Updated value: {}", mutable_variable);
    println!("The addition value is: {}", addition(3, 45));
    println!("The area of the circle: {}", area(2.23));

    string_slice();
    traits();
    lifetimes();
}

// Borrowing the variable to do something. But we cannot change the value of the variable.
fn do_something(borrowed_string: &String) {
    println!("The borrowed string is: {}", borrowed_string);
}

fn change_something(changed_string: &mut String) {
    changed_string.clear();
    changed_string.push_str("Updated string");
}

// Return type is Result<String, String> which means either its Ok or Err but in the form of String type.
fn read_file_content(file_name: &str) -> Result<String, String> {
    let file_text = fs::read_to_string(file_name);
    match file_text {
        // If the file is read successfully, then return the data otherwise return the error.
        Ok(data) => Ok(data),
        Err(err) => Err(format!("There was an error reading the file: {}", err)),
    }
}
