use std::sync::Arc;

// This function ONLY accepts &Vec<i32>
fn print_vec(v: &Vec<i32>) {
    println!("I received a &Vec<i32>: {:?}", v);
}

// Let's add a function that accepts &Arc<Vec<i32>>
fn print_arc(a: &Arc<Vec<i32>>) {
    println!("I received a &Arc<Vec<i32>>: {:?}", a);
}

fn main() {
    println!("=== Creating our data structures ===");
    let regular_vec = vec![1, 2, 3, 4, 5];
    let arc_vec = Arc::new(vec![1, 2, 3, 4, 5]);
    
    println!("\n=== Regular Vec examples ===");
    print_vec(&regular_vec);              // ✅ Works: &Vec<i32> matches exactly
    // print_arc(&regular_vec);           // ❌ Won't compile: regular_vec isn't an Arc
    
    println!("\n=== Arc<Vec> examples ===");
    // print_vec(&arc_vec);               // ❌ Won't compile: &Arc<Vec> ≠ &Vec
    print_vec(&*arc_vec);                 // ✅ Works: dereferencing Arc gets to Vec
    print_arc(&arc_vec);                  // ✅ Works: &Arc<Vec> matches exactly
    
    println!("\n=== Type inspection ===");
    println!("regular_vec type requires &Vec<i32>");
    println!("arc_vec type is Arc<Vec<i32>>");
    println!("&arc_vec type is &Arc<Vec<i32>>");
    println!("&*arc_vec type is &Vec<i32>");
    
    // Even though they print the same values:
    println!("\n=== Value printing ===");
    println!("arc_vec: {:?}", arc_vec);        // Arc<Vec> auto-derefs for display
    println!("&arc_vec: {:?}", &arc_vec);      // &Arc<Vec> auto-derefs for display
    println!("&*arc_vec: {:?}", &*arc_vec);    // &Vec displays directly
    
    // Let's see what happens with method calls
    println!("\n=== Method calls ===");
    println!("Length via arc_vec.len(): {}", arc_vec.len());          // Auto-derefs
    println!("Length via (*arc_vec).len(): {}", (*arc_vec).len());    // Manual deref
    println!("First element via arc_vec[0]: {}", arc_vec[0]);         // Auto-derefs
    println!("First element via (*arc_vec)[0]: {}", (*arc_vec)[0]);   // Manual deref
    
    // Real-world analogy in code:
    println!("\n=== Analogy with boxes ===");
    let box_of_vec = Arc::new(vec![1, 2, 3]);     // Box containing a vector
    print_arc(&box_of_vec);                        // Pass the whole box
    print_vec(&*box_of_vec);                       // Open box, pass contents
}