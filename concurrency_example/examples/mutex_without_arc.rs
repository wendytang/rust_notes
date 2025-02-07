use std::sync::Mutex;
use std::thread;

fn main() {
    println!("=== Example of what NOT to do ===");
    println!("This will fail to compile because Mutex alone isn't thread-safe!\n");

    // Try to use Mutex without Arc
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for thread_id in 0..5 {
        // This will fail because Mutex doesn't implement Send
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {}: counter = {}", thread_id, num);
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}