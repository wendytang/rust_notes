use std::sync::Arc;
use std::thread;

fn main() {
    // Create data in an Arc
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    // Spawn 3 threads that each print the data
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {} sees data: {:?}", i, *data_clone);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}