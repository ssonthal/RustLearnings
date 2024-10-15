// Java -> abstract classes
// JS -> Interfaces


/*
    public abstract class Summary {
        public String summarize();
    }

    // The diff is that traits can have default implementations of methods. In Java Abstract classes it can only have method definitions.
 */
pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("summarize");
    }
}

pub trait Fix {
    fn fix(&self) -> String {
        return String::from("fix it!");
    }
}

struct User {
    name: String, 
    age: u32
}

impl Fix for User {}

// can implement our defined traits for pre-defined Structs
// as well. 
impl Summary for String {}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

pub fn notify(item: &impl Summary){
    println!("{}", item.summarize());
}

// &impl Summary is a syntactic sugar for trait bounds

pub fn notify2<T: Summary + Fix>(item: T){
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let user = User{
        name: String::from("Shubham"),
        age: 25
    };
    notify2(user);
}
