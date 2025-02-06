use std::sync::Arc;

fn main() {
    // Create an Arc containing a vector
    let numbers = Arc::new(vec![1, 2, 3]);

    println!("\nLet's explore the memory layout of Arc:");
    println!("--------------------------------------");
    
    // 1. Print the Arc itself (shows the data it contains)
    println!("numbers: {:?}", numbers);
    
    // 2. Print the memory address OF the Arc smart pointer
    println!("&numbers: {:p}", &numbers);
    
    // 3. Print the Vec by dereferencing the Arc
    println!("*numbers: {:?}", *numbers);
    
    // 4. Print the memory address of the Vec inside the Arc
    println!("&*numbers: {:p}", &*numbers);
    
    println!("\nMemory Layout Explanation:");
    println!("------------------------");
    println!("1. &numbers  points to → Arc smart pointer");
    println!("2. Arc smart pointer    → contains pointer to heap data");
    println!("3. *numbers  gives us   → Vec[1, 2, 3] (dereferences Arc)");
    println!("4. &*numbers points to  → actual Vec in memory");
    
    // Create a clone to demonstrate Arc's shared ownership
    let numbers2 = Arc::clone(&numbers);
    println!("\nAfter cloning:");
    println!("-------------");
    println!("numbers2: {:?}", numbers2);
    println!("&numbers2: {:p}", &numbers2);        // Different Arc location
    println!("&*numbers2: {:p}", &*numbers2);      // Same Vec location!
}