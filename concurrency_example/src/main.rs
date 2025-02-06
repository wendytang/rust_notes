use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // ====== Example 1: Basic Arc Usage ======
    // Arc is used when you need to share ownership of data across multiple threads
    // Arc provides thread-safe reference counting
    
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    // Spawn multiple threads that read from the same data
    for i in 0..3 {
        let numbers_clone = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            println!("Thread {} accessing numbers: {:?}", i, *numbers_clone);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // ====== Example 2: Arc with Mutex ======
    // Mutex is used when you need mutable access to shared data
    // Arc + Mutex is a common pattern for shared mutable state
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn threads that increment the counter
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex to get mutable access
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // Mutex is automatically unlocked when num goes out of scope
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final counter value
    println!("Final counter value: {}", *counter.lock().unwrap());

    // ====== Example 3: Real-world scenario - Thread-safe Cache ======
    // This example shows how to implement a simple thread-safe cache
    
    struct Cache {
        data: Arc<Mutex<HashMap<String, String>>>
    }

    impl Cache {
        fn new() -> Self {
            Cache {
                data: Arc::new(Mutex::new(HashMap::new()))
            }
        }

        fn insert(&self, key: String, value: String) {
            let mut data = self.data.lock().unwrap();
            data.insert(key, value);
        }

        fn get(&self, key: &str) -> Option<String> {
            let data = self.data.lock().unwrap();
            data.get(key).cloned()
        }
    }
}

/*
When to use Arc:
1. When you need to share data between multiple threads
2. When you need multiple owners of the same data
3. When the data needs to outlive the thread that created it

When to use Mutex:
1. When you need mutable access to shared data
2. When you need to ensure only one thread can access data at a time
3. When you need to prevent race conditions

Common patterns:
1. Arc<T> - For sharing immutable data between threads
2. Arc<Mutex<T>> - For sharing mutable data between threads
3. Arc<RwLock<T>> - When you need multiple readers but exclusive writers

Best Practices:
1. Keep mutex locks as short as possible to avoid blocking
2. Use Arc only when necessary (within single thread, use Rc instead)
3. Consider using RwLock instead of Mutex when you have multiple readers
4. Be careful of deadlocks when using multiple mutexes
*/