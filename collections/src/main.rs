use std::collections::HashMap;

fn main() {
    // Initialize the vector.
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector is: {:?}", v);
    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    // The following line does not throw any error since the first variable is shadowed by the second
    // variable.
    let v = vec![1, 2, 3, 4, 5]; //The syntax specifies that the type of the vector will be determined based on the type of first value.
    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    for item in &v {
        println!("{}", item);
    }

    println!("Vector is: {:?}", v); // This line will throw an erro if we do not access the item through reference in the for loop above.
    
    // Define the hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores are: {:?}", scores);

    for (key, value) in &scores{
        println!("{key}: {value}")
    }
}
