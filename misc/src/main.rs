fn largest_num_fn(num_list: &[i32]) -> &i32 {
    let mut largest_num = &num_list[0];
    for item in num_list {
        if item > largest_num {
            largest_num = item;
        }
    }
    return largest_num;
}

fn main() {
    let my_list = vec![23, 345, 65, 76, 741];
    let my_list_sum = largest_num_fn(&my_list);
    println!("Hey the sum list is {}", my_list_sum);

    let my_list_iterator = my_list.iter();

    // Iterator does not required borrowing.
    for item in my_list_iterator {
        println!("The item is {}", item);
    }

    let my_list_data_squared: Vec<i32> = my_list
        .iter()
        .map(|x| x * x)
        .collect();
    println!("The squrared list is {:?}", my_list_data_squared); // mapping method to map some function to the elements of the array.

    assert_eq!(my_list_data_squared, vec![529, 119025, 4225, 5776, 549081]);
}
