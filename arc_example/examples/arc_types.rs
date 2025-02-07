use std::sync::Arc;

fn main() {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    
    // Demonstration of different ways to access the data
    println!("\n=== Different ways to access Arc data ===");
    println!("1. numbers (Arc itself): {:?}", numbers);          // Prints the Arc wrapper
    println!("2. &numbers (ref to Arc): {:p}", &numbers);       // Memory address of Arc
    println!("3. *numbers (deref to Vec): {:?}", *numbers);     // Dereferenced Arc (gets to Vec)
    println!("4. &*numbers (ref to Vec): {:p}", &*numbers);     // Memory address of Vec inside

    // Function that expects &Vec<i32>
    fn print_vec(v: &Vec<i32>) {
        println!("\nReceived &Vec<i32> at address: {:p}", v);
        println!("Contents: {:?}", v);
    }

    // Function that expects &Arc<Vec<i32>>
    fn print_arc(a: &Arc<Vec<i32>>) {
        println!("\nReceived &Arc<Vec<i32>> at address: {:p}", a);
        println!("Contents: {:?}", a);
    }

    println!("\n=== Function calls with different types ===");
    print_vec(&*numbers);  // Need to deref Arc to get &Vec
    print_arc(&numbers);   // Directly pass reference to Arc

    // Clone the Arc and show reference counting
    println!("\n=== Reference counting demonstration ===");
    println!("Initial ref count: {}", Arc::strong_count(&numbers));
    
    let numbers_clone = Arc::clone(&numbers);
    println!("After clone ref count: {}", Arc::strong_count(&numbers));
    
    drop(numbers_clone);
    println!("After drop ref count: {}", Arc::strong_count(&numbers));
}