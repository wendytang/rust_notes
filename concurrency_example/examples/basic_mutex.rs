use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a Mutex wrapped in an Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("=== Multiple threads incrementing a shared counter ===");
    
    // Spawn 5 threads that each increment the counter 10 times
    for thread_id in 0..5 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for i in 0..10 {
                // Lock the Mutex to get access to the data
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("Thread {} increment {}: counter = {}", thread_id, i, num);
                // Mutex is automatically unlocked when num goes out of scope
            }
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Final value should be 50 (5 threads × 10 increments)
    println!("\nFinal counter value: {}", *counter.lock().unwrap());
}