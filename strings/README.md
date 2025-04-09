# Rust Strings Tutorial

This guide explains the differences between `String` and `str` in Rust, and demonstrates common string operations.

## Key Concepts

1. **str (string slice)**
   - Immutable
   - Fixed-size
   - UTF-8 encoded
   - Dynamically sized type (DST)
   - Usually seen as `&str`
   - Stored in:
     * Binary (for string literals)
     * Stack or heap (when slicing a String)

2. **String**
   - Mutable
   - Growable
   - UTF-8 encoded
   - Owned
   - Heap-allocated
   - Similar to Vec<u8>

## Examples

Check the examples directory for demonstrations of:
1. `basic_strings.rs` - Basic string operations
2. `string_vs_str.rs` - Differences between String and str
3. `string_manipulation.rs` - Common string manipulation tasks
4. `string_conversion.rs` - Converting between String and str

## Common Operations

### Creating Strings
```rust
// String
let owned = String::from("hello");
let also_owned = "hello".to_string();
let empty = String::new();

// &str
let literal = "hello";  // type is &'static str
let slice: &str = &owned;  // borrowing from String
```

### Modification
```rust
// Only String can be modified
let mut owned = String::from("hello");
owned.push_str(" world");  // Works

let literal = "hello";
// literal.push_str(" world");  // Error! &str is immutable
```

### Concatenation
```rust
// Using String
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;  // Note: s1 is moved here

// Using format! macro (works with both)
let s = format!("{} {}", "hello", "world");
```

### Memory Considerations

1. **String**
   - Heap allocated
   - Can grow/shrink
   - More expensive
   - Owns its data

2. **&str**
   - Fixed size
   - Just a view into string data
   - Very cheap to copy (just copies the pointer)
   - Doesn't own its data

## When to Use Each

Use **&str** when:
- You only need to read string data
- You want to accept both String and &str (via deref coercion)
- You're writing function parameters
- You have string literals

Use **String** when:
- You need to own the string data
- You need to modify the string
- You're building/generating a string
- You need to store strings in a struct

## Common Patterns

1. **Function Parameters**
```rust
// Prefer &str for parameters
fn process(s: &str) { }  // Can accept both String and &str

// Not this
fn process(s: String) { }  // Can only accept String
```

2. **Struct Fields**
```rust
// If data is static or borrowed
struct Person<'a> {
    name: &'a str,
}

// If you need to own the data
struct Person {
    name: String,
}
```

3. **Returns**
```rust
// Return String when generating/building strings
fn build_string() -> String {
    format!("Hello, {}", "world")
}

// Return &str when returning string literals
fn get_static_str() -> &'static str {
    "Hello, world"
}
```

## Running the Examples

```bash
cd strings

# Run basic string operations example
cargo run --example basic_strings

# Run String vs str comparison
cargo run --example string_vs_str

# Run string manipulation examples
cargo run --example string_manipulation

# Run conversion examples
cargo run --example string_conversion
```