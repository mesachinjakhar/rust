use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        // for i in 0..10 {
        //     println!("hi number {i}, from the spawned thread!");
        //     thread::sleep(Duration::from_millis(1));
        // }
        println!("here is a vector: {v:?}");
    });

    handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread");
    //     thread::sleep(Duration::from_millis(1));
    // }

    
}
