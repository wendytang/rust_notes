# Arc (Atomic Reference Counting) Examples

This repository contains examples demonstrating the use of Arc in Rust for thread-safe reference counting.

## Running Examples

There are two ways to run the code:

1. Run the main example:
```bash
cd arc_example
cargo run
```

2. Run specific examples:
```bash
cd arc_example
cargo run --example simple_arc    # Demonstrates basic Arc usage
```

## Available Examples

### Main Example (`src/main.rs`)
- Demonstrates comprehensive Arc usage
- Shows thread spawning with shared data
- Includes memory management patterns

### Simple Arc (`examples/simple_arc.rs`)
- Basic demonstration of Arc
- Shows how to share data between threads
- Includes cloning and accessing Arc data

To run any specific example, use:
```bash
cargo run --example <example_name>
```

Where `<example_name>` is one of:
- `simple_arc`

## Key Concepts

1. Arc (Atomic Reference Counting)
   - Thread-safe reference counting
   - Safe sharing between threads
   - Memory management

2. Thread Safety
   - Safe concurrent access
   - No data races
   - Proper cleanup

## Code Structure
```
arc_example/
├── Cargo.toml
├── src/
│   └── main.rs           # Main Arc example
└── examples/
    └── simple_arc.rs     # Basic Arc usage example
```