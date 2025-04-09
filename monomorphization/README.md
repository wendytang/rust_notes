# Rust Monomorphization Examples

This directory contains examples demonstrating monomorphization in Rust.

## Running Examples

You can run the examples in two ways:

1. Using Cargo:
```bash
# Run basic monomorphization example
cargo run --example basic

# Run multiple type parameters example
cargo run --example multiple
```

2. Using rustc directly:
```bash
# For basic example
rustc examples/basic.rs
./basic

# For multiple example
rustc examples/multiple.rs
./multiple
```

## Available Examples

### 1. Basic Monomorphization (`examples/basic.rs`)
- Shows simple generic function usage
- Demonstrates turbofish syntax
- Shows type inference

### 2. Multiple Type Parameters (`examples/multiple.rs`)
- Shows generic struct with multiple type parameters
- Demonstrates type constraints
- Shows type name printing

## Key Concepts

1. **Monomorphization**: The process where generic code is converted into specific code for each concrete type at compile time.
2. **Turbofish Syntax**: Using `::<>` to explicitly specify generic type parameters.
3. **Type Inference**: Rust's ability to infer generic types from context.

## Code Structure
```
monomorphization/
├── Cargo.toml
├── src/
│   └── main.rs
└── examples/
    ├── basic.rs
    └── multiple.rs
```