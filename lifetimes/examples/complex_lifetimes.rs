// Example showing different lifetime parameters and their relationships
fn first_reference<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("y is: {}", y);  // We use y but return x's reference
    x  // Return type is tied to x's lifetime ('a), not y's lifetime ('b)
}

// Function that shows why we might need different lifetimes
fn store_and_print<'a, 'b>(primary: &'a str, temporary: &'b str) -> &'a str {
    println!("Temporary value: {}", temporary);  // temporary only needs to live until here
    primary  // primary needs to live longer, as we're returning it
}

fn main() {
    let long_lived_str = String::from("I live for a long time");
    let result;
    
    {
        let short_lived_str = String::from("I'll be gone soon");
        // Note: result's lifetime is tied to long_lived_str ('a), not short_lived_str ('b)
        result = store_and_print(&long_lived_str, &short_lived_str);
    }  // short_lived_str is dropped here, but that's okay because we only used 'b inside the function
    
    // We can still use result here because it's tied to long_lived_str's lifetime
    println!("Result: {}", result);

    // Another example showing the relationship
    let string1 = String::from("Hello");
    let result2;
    {
        let string2 = String::from("World");
        result2 = first_reference(&string1, &string2);  // string2 can be temporary
    }  // string2 goes out of scope
    println!("Can still print result2: {}", result2);  // This works because result2 is tied to string1's lifetime
}