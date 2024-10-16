

fn main() {
let longer;
   let str1 = String::from("Shubham");
   {
        let str2 = String::from("Sonthalia");
        longer = bigger_string_with_lifetimes(&str1, &str2);
        println!("{}", longer);
   }
   
    //    println!("bigger string is {}", bigger_string(str1, str2)); 
}

// returns a variable with the intersection of the lifetimes of variable str1 and str2.
// lifetime of str1 (from line 5 to line 13.)
// lifetime of str2 (from line 7 to line 10.)
fn bigger_string_with_lifetimes<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    return str2;
}

// works fine 

// fn bigger_string(str1: String, str2: String) -> String{
//     if str1.len() > str2.len() {
//         return str1;
//     }
//     return str2;
// }