use std::thread;
use std::time::Duration;
// 16.1 and 16.2 
// Fearless Concurrency
fn main() {
    // 1. waiting for the thread to finish and then running
    // the main thread.

    /* 
        let handle = thread::spawn(|| {
            for i in 1..10{
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
        // handle.join() lets us await the thread to finish before the execution continues. 
        handle.join().unwrap();
        for i in 1..5 {
            println!("hi number {i} from the main thread");
            thread::sleep(Duration::from_millis(1));
        }
    */

    // 2. runnign main and thread in parallel. 

    /*
        let handle = thread::spawn(|| {
            let mut c  = 0;
            for _i in 0..500000000{
                // println!("Hi {} from spawned thread", i);
                c = c + 1; 
                c = c - 1; 
            }
        });
        let mut c = 0; 
        for _i in 0..500000000 {
            // println!("Hi {} from main thread", i);
            c = c + 1; 
            c = c - 1; 
        }
        // waiting for the handler thread to finish the task.
        let _ = handle.join();

     */

    // closures with move to "move" ownership of the variable
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move || {
        println!("here's a vector {v:?}");
    });
    handler.join().unwrap();

}
