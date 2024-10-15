fn main() {
   let larger_int = larger(1, 2);
   let larger_char = larger('a','b');
   println!("{}", larger_int);
   println!("{}", larger_char);
}
// std::cmp:PartialOrd tells Rust that the T should be comparable with each other. If you put a = {msg: "hello"}, {msg: "world"}. Then it doesn't make any sense to compare them.
// To prevent this, we have explicitly mentioned this to Rust compiler

// The <T:...> block tells any compiler that hey, i will be using generics for this function from now on. Please allow me to do so. 

fn larger<T : std::cmp::PartialOrd>(a:T, b:T) -> T{
    if a > b {
        a
    }
    else{
        b
    }
}
