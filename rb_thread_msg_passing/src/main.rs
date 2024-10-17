use std::{sync::mpsc, thread::spawn};

pub fn calculate() -> i64{
    let mut ans:i64 = 0; 
    let rec = 
    {
        let (tx, rx) = mpsc::channel();
        for i in (0..10).step_by(2) {
            let tx1 = tx.clone();
            spawn(move || {
                let mut sum:i64 = 0; 
                for j in 10000000*i + 1..10000000*(i + 2) {
                    sum += j;
                }
                tx1.send(sum).unwrap();
            });
        }
        rx 
    };    
    for res in rec{
        ans += res;
    }
    return ans;
}


// mpsc : multiple producers single producer
fn main() {
    // sample on how to use message passing.

    /* 
        let (tx, rx) = mpsc::channel();
        spawn(move || {
            tx.send(String::from("hello wworld")).unwrap();
        });
        // should never unwrap coz if there's an error, 
        // the program will panic. 
        let value = rx.recv();
        match value {
            Ok(value) => println!("{}", value),
            Err(err) => println!("Error while reading the data -> {}", err)
        }
    */
    println!("ans: {}", calculate());
}
