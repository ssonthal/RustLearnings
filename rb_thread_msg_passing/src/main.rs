use std::{sync::mpsc, thread::spawn};


// mpsc : multiple producers single producer
fn main() {
    // sample on how to use 

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
    let mut ans = 0; 
    for res in rx{
        ans += res;
    }
    println!("{}", ans);
}
