// Example 1: Basic function with reference parameter
fn print_num(x: &i32) {  // Implicit lifetime
    println!("Number: {}", x);
}

// Example 2: Same function with explicit lifetime
fn print_num_explicit<'a>(x: &'a i32) {  // Explicit lifetime 'a
    println!("Number: {}", x);
}

// Example 3: Function with multiple references
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Example 1 & 2: Both implicit and explicit lifetime functions
    let number = 42;
    print_num(&number);
    print_num_explicit(&number);

    // Example 3: Function with lifetime relationships
    let string1 = String::from("short");
    let string2 = String::from("longer");
    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);

    // Example 4: Different scopes
    let string1 = String::from("hello");
    {
        let string2 = String::from("world");
        let temp_result = longest(&string1, &string2);
        println!("Temporary result: {}", temp_result);
    } // string2 goes out of scope here
}