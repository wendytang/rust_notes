use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    
    // Create multiple threads that share the data
    let mut handles = vec![];
    
    for id in 0..3 {
        let numbers_clone = Arc::clone(&numbers);

        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", id, numbers_clone);
            // Try accessing elements
            println!("Thread {} first element: {}", id, numbers_clone[0]);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Back in main thread: {:?}", numbers);
    println!("Reference count at end: {}", Arc::strong_count(&numbers));
}