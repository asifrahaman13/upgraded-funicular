fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("The value of y is {}", *y)
}
