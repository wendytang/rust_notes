// Common string manipulation tasks
fn main() {
    // 1. Basic Modifications
    println!("1. Basic Modifications:");
    let mut s = String::from("Hello");
    s.push_str(", ");  // append string slice
    s.push('W');       // append single character
    s.push_str("orld!");
    println!("Modified: {}", s);

    // 2. Case Conversion
    println!("\n2. Case Conversion:");
    let text = "Hello, World!";
    println!("Original: {}", text);
    println!("Uppercase: {}", text.to_uppercase());
    println!("Lowercase: {}", text.to_lowercase());

    // 3. Trimming
    println!("\n3. Trimming:");
    let text = "   Hello, World!   ";
    println!("Original: '{}'", text);
    println!("Trimmed: '{}'", text.trim());
    println!("Trim start: '{}'", text.trim_start());
    println!("Trim end: '{}'", text.trim_end());

    // 4. Splitting
    println!("\n4. Splitting:");
    let text = "Hello,World,How,Are,You";
    println!("Split by comma:");
    for part in text.split(',') {
        println!("  {}", part);
    }

    // 5. Joining
    println!("\n5. Joining:");
    let words = vec!["Hello", "World", "!"];
    let joined = words.join(" ");
    println!("Joined: {}", joined);

    // 6. Finding and Replacing
    println!("\n6. Finding and Replacing:");
    let text = "Hello, World!";
    println!("Contains 'World': {}", text.contains("World"));
    println!("Replaced: {}", text.replace("World", "Rust"));

    // 7. Substring
    println!("\n7. Substring:");
    let text = "Hello, World!";
    println!("First 5 chars: {}", &text[0..5]);
    
    // 8. String Building with format!
    println!("\n8. String Building:");
    let name = "World";
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);

    // 9. Chars and Bytes
    println!("\n9. Chars and Bytes:");
    let text = "Hello";
    println!("Characters:");
    for c in text.chars() {
        print!("{} ", c);
    }
    println!("\nBytes:");
    for b in text.bytes() {
        print!("{} ", b);
    }
    println!();
}