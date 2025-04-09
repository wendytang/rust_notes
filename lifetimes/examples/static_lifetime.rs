// Example demonstrating 'static lifetime
// 'static means the reference can live for the entire program duration

// 1. String literal - implicitly 'static
static GLOBAL_STRING: &str = "I'm a global string";

// 2. Struct containing a static reference
#[derive(Debug)]
struct StaticStr {
    content: &'static str
}

// 3. Trait bound requiring 'static lifetime
trait LivesForever: 'static {}

// 4. Function that returns a static string
fn get_static_str() -> &'static str {
    "I'm a static string literal"
}

// 5. Function accepting only 'static references
fn process_static(input: &'static str) {
    println!("Processing static str: {}", input);
}

fn main() {
    // 1. String literals are 'static
    let literal: &'static str = "I'm a string literal";
    println!("Literal: {}", literal);

    // 2. Using global static
    println!("Global: {}", GLOBAL_STRING);

    // 3. Struct with static reference
    let static_struct = StaticStr {
        content: "I live forever"
    };
    println!("Static struct: {:?}", static_struct);

    // 4. Getting a static string
    let static_str = get_static_str();
    println!("From function: {}", static_str);

    // 5. This works - string literal is 'static
    process_static("I'm static");

    // 6. This won't work - String is not 'static
    let owned_string = String::from("I'm not static");
    // process_static(&owned_string);  // Error!

    // 7. Demonstrating what doesn't work with 'static
    let string_on_heap = String::from("I'm on the heap");
    // let not_static: &'static str = &string_on_heap;  // Error!
    // Because string_on_heap will be dropped at the end of main

    // 8. Box::leak can create 'static references (but be careful!)
    let leaked_str: &'static str = Box::leak(Box::new(String::from("I've been leaked")));
    println!("Leaked static: {}", leaked_str);
}