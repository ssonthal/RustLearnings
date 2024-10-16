use std::thread;
use std::time::Duration;
// 16.1 and 16.2 
// Fearless Concurrency
fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10{
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // // handle.join() lets us await the thread to finish before the execution continues. 
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread");
    //     thread::sleep(Duration::from_millis(1));
    // }

    let handle = thread::spawn(|| {
        for i in 0..5{
            println!("Hi {} from spawned thread", i);
        }
    });
    for i in 0..5 {
        println!("Hi {} from main thread", i);
    }

    let _ = handle.join();
}
