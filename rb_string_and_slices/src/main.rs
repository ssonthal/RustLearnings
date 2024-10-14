fn main() {
   // three kinds of strings in rust
   // 1. String::from("Hello World") => owner of the memory 
   // location.
   // 2. &str[0..5] => a reference to a contigous memory location
   // 3. string literals => let str = "hello";

   let word = String::from("Hello world");
   let first_word = get_first_name(word);
   println!("First name from String::from logic is {}", first_word);
   
   let word2 = String::from("Hello World");
   let first_word_second = get_first_name_second(&word2);
   println!("First word from string slices - {}", first_word_second); 

    let word3 = "hello";
    println!("string literal {}", word3);


}

fn get_first_name(str:String) -> String{
    let mut ans = String::from("");
    for c in str.chars(){
        if c == ' '{
            break;
        }
        ans.push_str(&c.to_string());
    }
    return ans
}

fn get_first_name_second(str: &String) -> &str {
    let mut idx = 0;
    for (i, c) in str.chars().enumerate(){
        if c == ' '{
            idx = i;
            break;
        }
    }
    return &str[0..idx];
}
