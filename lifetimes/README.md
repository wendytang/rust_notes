# Rust Lifetimes Examples

This directory contains examples demonstrating lifetime parameters in Rust.

## What are Lifetimes?

Lifetimes are Rust's way of ensuring that references are valid for the time they're being used. They are:
- A type of generic parameter
- Denoted with a single quote (') followed by a name
- Used to tell the compiler how references relate to each other

## Running Examples

```bash
# Run basic lifetimes example
cargo run --example basic_lifetimes

# Run struct lifetimes example
cargo run --example struct_lifetimes
```

## Examples Explained

### 1. Basic Lifetimes (`examples/basic_lifetimes.rs`)

Shows three key concepts:
```rust
// 1. Implicit (elided) lifetimes
fn print_num(x: &i32) { }

// 2. Explicit lifetimes
fn print_num_explicit<'a>(x: &'a i32) { }

// 3. Multiple references with related lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { }
```

### 2. Struct Lifetimes (`examples/struct_lifetimes.rs`)

Shows how to:
- Use lifetimes with structs
- Implement methods on structs with lifetimes
- Store references in structs safely

## Key Concepts

1. **Lifetime Parameters**
   - Are generic parameters
   - Start with a single quote (')
   - Help the compiler track reference validity

2. **Lifetime Elision**
   - Compiler can often figure out lifetimes
   - Three rules for automatic inference
   - Can be explicitly written when needed

3. **Lifetime Relationships**
   - References must not outlive their data
   - Output lifetimes relate to input lifetimes
   - Structs can hold references with lifetimes

## Common Patterns

1. **Function Arguments**
```rust
fn foo<'a>(x: &'a i32)           // One reference
fn foo<'a>(x: &'a i32, y: &'a i32) // Multiple references
```

2. **Struct Definitions**
```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

3. **Implementation Blocks**
```rust
impl<'a> Excerpt<'a> {
    fn method(&self) -> &str { }
}
```