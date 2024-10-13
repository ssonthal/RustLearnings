
// use core::fmt;
// use std::{fmt::{Debug, Formatter}, fs};

// #[derive(Clone)]
// struct User {
//     active: bool,
//     sign_in_count: u64,
//     username: String,
// }

// enum Shape {
//     Rectangle(f64, f64),
//     Circle (f64),
//     Square (f64)
// }

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }


// pub fn calculate_area(shape:Shape) -> f64{
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
//         Shape::Square(side) => side*side,
//         Shape::Rectangle(width, height) => width*height,
//     }
// }

fn main() {
    // let square = Shape::Square(5.0);
    // let circle = Shape::Circle(5.0);
    // let rectangle = Shape::Rectangle(2.0, 5.0);

    // println!("Area of square: {}", calculate_area(square));
    // println!("Area of circle: {}", calculate_area(circle));
    // println!("Area of recatangle: {}", calculate_area(rectangle));

    // let greeting_file_result = fs::read_to_string("hello.txt");
    
    // 1. Unwrapping directly

     // print!("{}", greeting_file_result.unwrap());


    // 2. matching and exiting cleanly
    // match greeting_file_result {
    //     Ok(file_content) => {
    //         println!("File read successfully: {}", file_content);
    //     },
    //     Err(error) => {
    //         println!("Failed to read file: {}", error);
    //     }
    // }

    // let my_string = String::from("raman");
    // match find_first_a(my_string) {
    //     Some(index) => println!("The letter 'a' is found at index: {}", index),
    //     None => println!("The letter 'a' is not found in the string."),
    // }

    let v1 = vec![1, 2, 3];

    // let v1_iter = v1.into_iter();

    for mut val in v1 {
        val = val + 1; 
        println!("{}", val);
    }

    // for val in v1_iter {
    //     println!("{}", val);
    // }
}

// fn change_name(user1: User) {
//     print!("User 1 username: {:?}", user1.active);
// }