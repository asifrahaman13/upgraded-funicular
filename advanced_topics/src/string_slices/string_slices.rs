pub fn string_slice() {
    println!("Hello this is for the string types");

    // Slices are generic term. Slice can be of string vector etc.
    let my_string = String::from("niander my code");
    let first_word = extract_first_word(&my_string);
    println!("The first word from the string is : {}", first_word);

    // The difference is that the following string will be converted in the form of binary.
    let another_string = "This is another typed string";
    println!("The another string is: {}", another_string)
}

// A better method to save the memory on the heap as no duplication of the string occures.
// String slices are view to the string data type.
// If nothing is mentioned then also it is a string slice type by default.
fn extract_first_word(my_string: &String) -> &str {
    let mut space_index = 0;
    for item in my_string.chars() {
        if item == ' ' {
            break;
        }
        space_index = space_index + 1;
    }
    return &my_string[0..space_index];
}
