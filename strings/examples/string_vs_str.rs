// Demonstrating key differences between String and str
fn takes_slice(slice: &str) {
    println!("Got slice: {}", slice);
}

fn takes_string(string: String) {
    println!("Got string: {}", string);
}

// Function that returns a string literal
fn return_str() -> &'static str {
    "I'm a &str"
}

// Function that returns an owned String
fn return_string() -> String {
    String::from("I'm a String")
}

fn main() {
    println!("1. Creation and Memory:");
    // str (string slice)
    let literal = "I'm a &str";  // stored in program binary
    println!("Literal: {}", literal);

    // String
    let owned = String::from("I'm a String");  // stored on heap
    println!("Owned: {}", owned);

    println!("\n2. Mutability:");
    // String can be modified
    let mut owned = String::from("Hello");
    owned.push_str(" World");
    println!("Modified String: {}", owned);

    // str cannot be modified
    let literal = "Hello";
    // literal.push_str(" World");  // This would not compile

    println!("\n3. Function Parameters:");
    // &str is more flexible - can take both &str and &String
    takes_slice("literal str");
    takes_slice(&owned);  // &String coerces to &str

    // String is less flexible
    takes_string(String::from("new string"));
    // takes_string(literal);  // This would not compile
    takes_string(literal.to_string());  // Need to convert

    println!("\n4. Returns:");
    println!("{}", return_str());
    println!("{}", return_string());

    println!("\n5. Size Knowledge:");
    // String has known size (pointer + capacity + length)
    let owned = String::from("hello");
    println!("String size: {} bytes", std::mem::size_of_val(&owned));

    // &str is a fat pointer (pointer + length)
    let slice = "hello";
    println!("&str size: {} bytes", std::mem::size_of_val(&slice));

    println!("\n6. Heap Allocation:");
    // String allocates on heap
    let _owned = String::from("hello");  // allocates memory

    // &str just references existing data
    let _literal = "hello";  // no allocation
}