# Understanding Mutex in Rust

## What is Mutex?
- **Mutex** = **Mut**ual **Ex**clusion
- A synchronization primitive that ensures only one thread can access data at a time
- Like a lock on data that only one thread can hold at once
- Other threads must wait until the lock is released

## Mutex + Arc Pattern
```rust
use std::sync::{Arc, Mutex};

// Common pattern for shared, mutable state across threads
let shared_data = Arc::new(Mutex::new(0));

// Clone Arc for new thread
let data_clone = Arc::clone(&shared_data);
```

## Why use Arc with Mutex?
- **Mutex alone**: Provides synchronization but isn't thread-safe for sharing
- **Arc alone**: Provides sharing but no mutation
- **Arc<Mutex<T>>**: Combines thread-safe sharing with synchronized mutation

## Basic Usage
```rust
// Create shared counter
let counter = Arc::new(Mutex::new(0));

// In thread
let mut num = counter.lock().unwrap();  // Lock and get data
*num += 1;                              // Modify data
// Lock automatically releases when num goes out of scope
```

## Common Patterns

### 1. Simple Counter
```rust
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});
```

### 2. Protected Data Structure
```rust
struct BankAccount {
    balance: i32,
    transactions: Vec<String>,
}

let account = Arc::new(Mutex::new(BankAccount {
    balance: 100,
    transactions: Vec::new(),
}));
```

## Best Practices

### Do's:
- ✅ Keep lock periods as short as possible
- ✅ Release locks (automatically by dropping guard) promptly
- ✅ Use Arc<Mutex<T>> for shared mutable state between threads
- ✅ Consider RwLock for many-readers/few-writers scenarios

### Don'ts:
- ❌ Hold locks across await points or long operations
- ❌ Create nested locks (risk of deadlock)
- ❌ Use Mutex without Arc for cross-thread sharing
- ❌ Clone the Mutex itself (clone the Arc instead)

## When to Use Mutex

### Use Mutex when you need:
1. Shared mutable state between threads
2. Synchronized access to data
3. To prevent data races
4. To ensure sequential access to resources

### Consider alternatives when:
1. You only need read access (use RwLock)
2. You don't need shared access (use regular variables)
3. You have single-threaded code (no synchronization needed)

## Common Gotchas

### 1. Deadlocks
```rust
// DON'T DO THIS:
let mut lock1 = mutex1.lock().unwrap();
let mut lock2 = mutex2.lock().unwrap();  // Potential deadlock!

// DO THIS:
// Acquire locks in consistent order
// Or use parking_lot::Mutex for deadlock detection
```

### 2. Poisoned Mutex
```rust
// Handle poisoned mutex (when a thread panicked while holding lock)
let mut guard = match mutex.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(),
};
```

## Performance Considerations
- Mutex has overhead - don't use for simple operations
- Lock contention can impact performance
- Consider atomic types for simple counters
- Use RwLock when you have many readers

## Real World Example
```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Shared resource
struct Counter {
    count: i32,
    last_updated: String,
}

fn main() {
    let counter = Arc::new(Mutex::new(Counter {
        count: 0,
        last_updated: String::new(),
    }));

    let mut handles = vec![];

    // Spawn multiple threads
    for id in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut data = counter.lock().unwrap();
            data.count += 1;
            data.last_updated = format!("Updated by thread {}", id);
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Remember
- Mutex is for **synchronization**
- Arc is for **sharing**
- Together they enable **shared mutable state** across threads
- Always prefer simpler solutions if you don't need shared mutable state