
// when the name variable goes out of scope
// the struct should also go out of scope.
//that is why it's important to give the 
// lifetime of the struct.
struct User<'a, 'b> {
    first_name : &'a str,
    last_name: &'b str
}

fn main() {
    // dangling pointer. bad coding practice. 

    // {
    //         let string = String::from("Shubham");
    //         let user = User {name:&string};
    // }
    // println!("{}", user.name);
    let first_name = String::from("");
    let last_name = String::from("Sonthalia");
    let user = User {first_name: &first_name, last_name: &last_name};
    println!("Full Name: {} {}", user.first_name, user.last_name);
}
