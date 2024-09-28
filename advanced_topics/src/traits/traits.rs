pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("The user is {}. And his age is: {}", self.name, self.age);
    }
}

pub fn traits() {
    println!("This is the trait");

    let my_user = User {
        name: String::from("Jonathon smith"),
        age: 23,
    };

    let summarize = my_user.summarize();

    println!("The summarization is: {}", summarize);

    notify(my_user);
}

fn notify(u: impl Summary) {
    println!("{}", u.summarize());
}