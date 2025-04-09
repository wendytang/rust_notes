# Static Lifetimes in Rust

This guide explains when to use and when not to use `'static` lifetimes in Rust.

## What is 'static?

A `'static` lifetime means the data lives for the entire duration of the program. It's the longest possible lifetime and is often used for:
- String literals
- Constants
- Static variables
- Items stored in the program's binary

## When to Use 'static

1. **Configuration Constants**
```rust
static CONFIG_PATH: &str = "/etc/app/config";
static MAX_CONNECTIONS: u32 = 100;

struct Config {
    app_name: &'static str,    // Good: Configuration won't change
    version: &'static str,     // Good: Version is fixed
}
```

2. **Error Messages and Static Text**
```rust
fn get_error() -> &'static str {
    "An error occurred"        // Good: Error message is fixed
}

static ERROR_MESSAGES: &[&str] = &[
    "Not found",
    "Access denied",
];
```

3. **Global Constants**
```rust
const PI: f64 = 3.14159;
static APP_NAME: &str = "MyApp";
```

4. **Type Bounds in Traits**
```rust
trait MyTrait: 'static {}  // Indicates type must own its data
```

## When NOT to Use 'static

1. **Dynamic or User Data**
```rust
// BAD:
struct User {
    name: &'static str,  // Too restrictive!
}

// GOOD:
struct User {
    name: String,        // Owns the string
}
// OR:
struct User<'a> {
    name: &'a str,      // Borrows with flexible lifetime
}
```

2. **Function Parameters**
```rust
// BAD:
fn process_name(name: &'static str) {
    // Too restrictive - can only accept string literals
}

// GOOD:
fn process_name(name: &str) {
    // Can accept any string reference
}
```

3. **Temporary or Generated Data**
```rust
// BAD:
let username = format!("user_{}", id);
let static_ref: &'static str = &username;  // Won't work!

// GOOD:
let username = format!("user_{}", id);
let user = User {
    name: username,  // Take ownership
};
```

4. **Most Struct Fields**
```rust
// BAD:
struct Article {
    title: &'static str,    // Too restrictive
    content: &'static str,  // Too restrictive
}

// GOOD:
struct Article {
    title: String,          // Owned
    content: String,        // Owned
}
```

## Common Patterns and Best Practices

1. **For Configuration**
```rust
// Good: Static configuration
static CONFIG: Config = Config {
    name: "MyApp",
    version: "1.0.0",
};

// Bad: Dynamic configuration
static mut SETTINGS: &str = "";  // Avoid mutable statics
```

2. **For Application Data**
```rust
// Good: Flexible ownership
struct User {
    name: String,          // Owned string
    temp_token: &'a str,   // Borrowed with lifetime
    role: &'static str,    // Static only for fixed values
}
```

3. **For Function Returns**
```rust
// Good: Static for fixed strings
fn get_version() -> &'static str {
    "1.0.0"
}

// Good: Owned for dynamic strings
fn get_username() -> String {
    format!("user_{}", generate_id())
}
```

## Key Takeaways

1. Use `'static` for:
   - Program constants
   - Fixed configuration
   - Error messages
   - Truly static data

2. Avoid `'static` for:
   - User input
   - Dynamic data
   - Most struct fields
   - Function parameters (unless specifically needed)

3. Better alternatives:
   - `String` for owned data
   - `&str` for borrowed data
   - `&'a str` for generic lifetimes

## Examples

Check the examples directory for demonstrations of:
- Proper static usage
- Common patterns
- Alternative approaches

Remember: When in doubt, prefer owned types (`String`) or generic lifetimes (`&'a str`) over `'static`. Only use `'static` when you truly need data to live for the entire program duration.