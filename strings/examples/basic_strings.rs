// Basic string operations and differences between String and str
fn main() {
    // 1. String Creation
    let string_literal = "Hello, world";  // type: &'static str
    let string_owned = String::from("Hello, world");  // type: String
    let string_to = "Hello, world".to_string();  // type: String
    
    println!("1. Different ways to create strings:");
    println!("Literal (&str): {}", string_literal);
    println!("String::from: {}", string_owned);
    println!("to_string(): {}", string_to);

    // 2. String Modification
    let mut mutable_string = String::from("Hello");
    mutable_string.push_str(", world");  // Works with String
    println!("\n2. Modified string: {}", mutable_string);

    // This wouldn't work with &str:
    // let mut literal = "Hello";
    // literal.push_str(", world");  // Error! &str is immutable

    // 3. String Length
    let text = "Hello, world";
    println!("\n3. String lengths:");
    println!("Length (bytes): {}", text.len());  // length in bytes
    println!("Length (chars): {}", text.chars().count());  // length in characters

    // 4. String Slicing
    let hello_world = String::from("Hello, world!");
    let hello = &hello_world[0..5];  // type: &str
    println!("\n4. Sliced string: {}", hello);

    // 5. String Concatenation
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    
    // Using +
    let hello_world = hello + &world;  // Note: hello is moved here
    println!("\n5. Concatenated: {}", hello_world);
    
    // Using format! macro
    let hello = "Hello, ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("format! macro: {}", hello_world);

    // 6. String Iteration
    let text = "Hello";
    println!("\n6. String iteration:");
    
    println!("Bytes:");
    for b in text.bytes() {
        print!("{} ", b);
    }
    
    println!("\nChars:");
    for c in text.chars() {
        print!("{} ", c);
    }
    println!();
}