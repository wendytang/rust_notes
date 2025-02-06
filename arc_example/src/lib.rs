use std::sync::Arc;
use std::thread;

fn main() {
    // Create an Arc containing a vector
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    
    // Demonstration of different ways to access the data
    println!("1. numbers (Arc itself): {:?}", numbers);          // Prints the Arc wrapper
    println!("2. &numbers: {:?}", &numbers);                     // Reference to Arc
    println!("3. *numbers: {:?}", *numbers);                     // Dereferenced Arc (gets to Vec)

    let mut handles = vec![];

    for i in 0..3 {
        // Clone the Arc - this increments the reference count
        let numbers_clone = Arc::clone(&numbers);  // &numbers because Arc::clone takes a reference

        let handle = thread::spawn(move || {
            // Different ways to access the data inside numbers_clone
            println!("Thread {}:", i);
            println!("  a. numbers_clone: {:?}", numbers_clone);        // Prints Arc wrapper
            println!("  b. &numbers_clone: {:?}", &numbers_clone);      // Reference to Arc
            println!("  c. *numbers_clone: {:?}", *numbers_clone);      // Gets to the Vec inside
            
            // You can also access individual elements
            println!("  d. First element: {}", (*numbers_clone)[0]);    // Dereference then index
            println!("  e. Also first element: {}", numbers_clone[0]);  // Implicit deref via Index trait
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/*
Key Points:

1. Arc::clone(&numbers):
   - Takes a reference to the Arc (&)
   - Creates a new handle to the same data
   - Increments the reference count

2. *numbers_clone:
   - Dereferences the Arc to access the Vec inside
   - Necessary when you need the actual vector, not the Arc wrapper
   - Can be implicit in some cases due to Rust's auto-dereferencing

3. Common Patterns:
   - Arc::clone(&arc) - Creating new handle
   - *arc_clone     - Getting to the inner data
   - arc_clone[0]   - Implicit dereferencing via Index trait
*/